use basket_defs::command::Command;
use rkyv::AlignedVec;

pub fn serialize_command(command: &Command) -> AlignedVec {
    println!("{:#?}", command);
    rkyv::to_bytes::<_, 256>(command).unwrap()
}
