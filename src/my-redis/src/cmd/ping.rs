use crate::{Connection, Frame, Parse, ParseError};
use bytes::Bytes;
use tracing::instrument;

#[derive(Debug, Default)]
pub struct Ping {
    msg: Option<String>,
}

impl Ping {
    pub fn new(msg: Option<String>) -> Self {
        Self { msg }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Ping> {
        match parse.next_string() {
            Ok(msg) => Ok(Ping::new(Some(msg))),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let response = match self.msg {
            None => Frame::Simple("PONG".to_string()),
            Some(msg) => Frame::Bulk(Bytes::from(msg)),
        };
        dst.write_frame(&response).await?;

        Ok(())
    }
    pub(crate) fn into_frame(self) -> Frame {
      let mut frame = Frame::array();
      frame.push_bulk(Bytes::from("ping".as_bytes()));
      if let Some(msg) = self.msg {
          frame.push_bulk(Bytes::from(msg));
      }
      frame
  }
}
