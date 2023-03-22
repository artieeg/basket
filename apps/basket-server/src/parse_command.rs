use crate::{command::Command, buffer::Buffer};
use anyhow::Result;
use rkyv::Deserialize;

pub fn parse_command(buffer: &Buffer) -> Result<Command> {
    let archived = unsafe { rkyv::archived_root::<Command>(&buffer[..]) };

    let command: Command = archived.deserialize(&mut rkyv::Infallible)?;

    Ok(command)
}
