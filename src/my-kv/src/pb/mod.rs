pub mod abi;
use abi::{command_request::RequestData, *};
use bytes::Bytes;
use http::StatusCode;
use prost::Message;

use crate::KvError;

impl CommandRequest {
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        let h_get = Hget {
            table: table.into(),
            key: key.into(),
        };
        Self {
            request_data: Some(RequestData::Hget(h_get)),
        }
    }
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        let h_get_all = Hgetall {
            table: table.into(),
        };
        Self {
            request_data: Some(RequestData::Hgetall(h_get_all)),
        }
    }
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        let h_set = Hset {
            table: table.into(),
            pair: Some(Kvpair::new(key, value)),
        };
        Self {
            request_data: Some(RequestData::Hset(h_set)),
        }
    }
    pub fn new_hmget(table: impl Into<String>, keys: Vec<impl Into<String>>) -> Self {
        let c = keys.into_iter().map(|v| v.into()).collect::<Vec<String>>();
        let m_hmget = Hmget {
            table: table.into(),
            keys: c,
        };
        Self {
            request_data: Some(RequestData::Hmget(m_hmget)),
        }
    }
    pub fn new_hexist(table: impl Into<String>, key: impl Into<String>) -> Self {
        let h_exist = Hexist {
            table: table.into(),
            key: key.into(),
        };
        Self {
            request_data: Some(RequestData::Hexist(h_exist)),
        }
    }
    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        let s = Hdel {
            table: table.into(),
            key: key.into(),
        };
        Self {
            request_data: Some(RequestData::Hdel(s)),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        let s = String::from(v);
        s.into()
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(v)),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(buf: &[u8; N]) -> Self {
        Bytes::copy_from_slice(&buf[..]).into()
    }
}

impl From<Bytes> for Value {
    fn from(buf: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(buf)),
        }
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = KvError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let msg = Value::decode(value)?;
        Ok(msg)
    }
}
impl TryFrom<Value> for Vec<u8> {
    type Error = KvError;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let mut buf = Vec::with_capacity(v.encoded_len());
        v.encode(&mut buf)?;
        Ok(buf)
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(origin: (String, Value)) -> Self {
        Kvpair::new(origin.0, origin.1)
    }
}

impl From<bool> for CommandResponse {
    fn from(v: bool) -> Self {
        let mut res = Self {
            status: StatusCode::OK.as_u16() as _,
            ..Default::default()
        };
        if !v {
            res.status = StatusCode::NOT_FOUND.as_u16() as _;
        }
        res
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}
impl From<Vec<Value>> for CommandResponse {
    fn from(values: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values,
            ..Default::default()
        }
    }
}
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        };
        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }
        result
    }
}
