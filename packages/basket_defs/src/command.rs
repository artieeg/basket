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
// This will generate a PartialEq impl between our unarchived and archived types
#[archive(compare(PartialEq))]
// To use the safe API, you have to derive CheckBytes for the archived type
#[archive_attr(derive(CheckBytes, Debug))]
pub struct Command {
    pub action: Action,
    pub timestamp: u128,
}
