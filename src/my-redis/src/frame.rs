use std::{fmt, io::Cursor, num::TryFromIntError, string::FromUtf8Error};

use bytes::{Buf, Bytes};

#[derive(Clone, Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    Incomplete,
    Other(crate::Error),
}

impl Frame {
    pub(crate) fn array() -> Self {
        Self::Array(vec![])
    }

    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        use Frame::*;
        match self {
            Array(arr) => arr.push(Bulk(bytes)),
            _ => panic!("not an array frame"),
        }
    }
    pub(crate) fn push_int(&mut self, value: u64) {
        use Frame::*;
        match self {
            Array(arr) => arr.push(Integer(value)),
            _ => panic!("not an array frame"),
        }
    }
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            b'+' => {
                get_line(src)?;
                Ok(())
            }
            b'-' => {
                get_line(src)?;
                Ok(())
            }
            b':' => {
                get_decimal(src)?;
                Ok(())
            }
            b'$' => {
                //多行字符串 $-1\r\n\r\n 表示空
                if b'-' == peek_u8(src)? {
                    skip(src, 4)
                } else {
                    let len: usize = get_decimal(src)?.try_into()?;
                    skip(src, len + 2)
                }
            }
            b'*' => {
                let len = get_decimal(src)?;
                for _ in 0..len {
                    Frame::check(src)?;
                }
                Ok(())
            }
            actual => Err(format!("protocol error; invalid frame type byte `{}`", actual).into()),
        }
    }


    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
      match get_u8(src)? {
       b'+' => {
        let line = get_line(src)?.to_vec();
        let res = String::from_utf8(line)?;
        Ok(Frame::Simple(res))
       },
       b'-' => {  
        let line = get_line(src)?.to_vec();
        let res = String::from_utf8(line)?;
        Ok(Frame::Error(res))
       },
       b':' => {
        let len = get_decimal(src)?;
        Ok(Frame::Integer(len))

       },
       b'$' => {
        if b'-' == peek_u8(src)? {
          let line = get_line(src)?;
          if line != b"-1" {
            return Err("protocol error; invalid frame format".into());
          }
          Ok(Frame::Null)
        } else {
          let len = get_decimal(src)?.try_into()?;
          let n = len + 2;
          if src.remaining() < n {
            return Err(Error::Incomplete)
          }
          let data = Bytes::copy_from_slice(&src.chunk()[..len]);
          skip(src, n)?;
          Ok(Frame::Bulk(data))
        }
       },
       b'*' => {
        let len = get_decimal(src)?.try_into()?;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
          out.push(Frame::parse(src)?);
        }
        Ok(Frame::Array(out))
       },
       _ => unimplemented!()
      }
    }

    pub(crate) fn to_error(&self) -> crate::Error {
      format!("unexpected frame: {}", self).into()
    }
}

impl PartialEq<&str> for Frame {
    fn eq(&self, other: &&str) -> bool {
        use Frame::*;
        match self {
            Simple(s) => s.eq(other),
            Bulk(s) => s.eq(other),
            _ => false,
        }
    }
}
impl fmt::Display for Frame {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      use std::str;

      match self {
          Frame::Simple(response) => response.fmt(fmt),
          Frame::Error(msg) => write!(fmt, "error: {}", msg),
          Frame::Integer(num) => num.fmt(fmt),
          Frame::Bulk(msg) => match str::from_utf8(msg) {
              Ok(string) => string.fmt(fmt),
              Err(_) => write!(fmt, "{:?}", msg),
          },
          Frame::Null => "(nil)".fmt(fmt),
          Frame::Array(parts) => {
              for (i, part) in parts.iter().enumerate() {
                  if i > 0 {
                      write!(fmt, " ")?;
                      part.fmt(fmt)?;
                  }
              }

              Ok(())
          }
      }
  }
}
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    let start = src.position() as usize;
    let end = src.get_ref().len();
    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }
    Err(Error::Incomplete)
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    use atoi::atoi;
    let line = get_line(src)?;
    atoi::<u64>(line).ok_or_else(|| "protocol error; invalid frame format".into())
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.chunk()[0])
}
fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn skip(src: &mut Cursor<&[u8]>, n: usize) -> Result<(), Error> {
    if src.remaining() < n {
        return Err(Error::Incomplete);
    }
    src.advance(n);
    Ok(())
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value.into())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        "protocol error; invalid frame format".into()
    }
}
impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        "protocol error; invalid frame format".into()
    }
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(f),
            Error::Other(err) => err.fmt(f),
        }
    }
}
