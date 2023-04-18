pub mod abi;

use abi::{command_request::RequestData, *};

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        let h_set = Hset {
            table: table.into(),
            pair: Some(Kvpair::new(key, value)),
        };
        Self {
            request_data: Some(RequestData::Hset(h_set)),
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
