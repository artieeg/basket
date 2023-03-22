mod buffer;
mod parse_command;

use crate::buffer::{Buffer, BufferCapacity};

use crate::parse_command::parse_command;

use anyhow::Result;
use buffer::BUFFER_CAPACITY;
use log::info;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

fn main() -> Result<()> {
    env_logger::init();

    let mut buffer = [0; BUFFER_CAPACITY as usize];
    let listener = TcpListener::bind((HOST, PORT)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            info!("New connection {:?}", stream);

            let command_len = read_command_from_stream(&mut stream, &mut buffer)?;

            let command = parse_command(&buffer.split_at(command_len.into()).0);

            println!("Command {:#?}", command);
        }
    }

    Ok(())
}

fn read_command_from_stream(stream: &mut TcpStream, buffer: &mut Buffer) -> std::io::Result<BufferCapacity> {
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
