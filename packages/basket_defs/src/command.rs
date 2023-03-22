use rkyv::{Archive, Deserialize, Serialize};
use bytecheck::CheckBytes;

pub type Key = u32;

#[derive(Archive, Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum Value {
    INT(i32),
}

#[derive(Archive, Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum Action {
    SET(Key, Value),
    GET(Key),
    DEL(Key),
}

#[derive(Archive, Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct Command {
    pub action: Action,
    pub timestamp: u128,
}
