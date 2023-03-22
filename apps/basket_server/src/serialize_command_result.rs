use basket_defs::command_result::CommandResult;
use rkyv::AlignedVec;

pub fn serialize_command_result(result: &CommandResult) -> AlignedVec {
    rkyv::to_bytes::<_, 256>(result).unwrap()
}
