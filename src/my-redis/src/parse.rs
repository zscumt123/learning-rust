use crate::Frame;
use bytes::Bytes;
use std::{fmt, str, vec};

#[derive(Debug)]
pub(crate) struct Parse {
    parts: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    EndOfStream,
    Other(crate::Error),
}

impl Parse {
    pub(crate) fn new(frame: Frame) -> Result<Parse, ParseError> {
        let arr = match frame {
            Frame::Array(val) => val,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };
        Ok(Parse {
            parts: arr.into_iter(),
        })
    }

    fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfStream)
    }
    pub(crate) fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )
            .into()),
        }
    }
    pub(crate) fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )
            .into()),
        }
    }
    pub(crate) fn next_init(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;

        const MSG: &str = "protocol error; invalid number";
        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(s) => atoi::<u64>(s.as_bytes()).ok_or_else(|| MSG.into()),
            Frame::Bulk(data) => atoi::<u64>(&data).ok_or_else(|| MSG.into()),
            frame => Err(format!("protocol error; expected int frame but got {:?}", frame).into()),
        }
    }
    pub(crate) fn finish(&mut self) -> Result<(), ParseError> {
      if self.parts.next().is_none() {
        Ok(())
      } else {
        Err("protocol error; expected end of frame, but there was more".into())
      }
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
