use basket_defs::command::Command;
use anyhow::Result;
use rkyv::{Deserialize, AlignedVec};

pub fn parse_command(buffer: &[u8]) -> Result<Command> {
    let mut aligned_vec = AlignedVec::new();
    aligned_vec.extend_from_slice(buffer);

    let archived = unsafe { rkyv::archived_root::<Command>(&aligned_vec[..]) };

    let command: Command = archived.deserialize(&mut rkyv::Infallible)?;

    Ok(command)
}
