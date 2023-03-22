use crate::buffer::{Buffer, BufferCapacity, BUFFER_CAPACITY};
use crate::TcpStream;
use std::io::Read;

pub fn read_command_from_stream(
    stream: &mut TcpStream,
    buffer: &mut Buffer,
) -> std::io::Result<BufferCapacity> {
    let mut size_bytes = [0; 2];
    stream.read(&mut size_bytes)?;

    let len = BufferCapacity::from_be_bytes(size_bytes);

    if len >= BUFFER_CAPACITY {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer Overflow",
        ));
    }

    stream.read(buffer)?;

    Ok(len)
}
