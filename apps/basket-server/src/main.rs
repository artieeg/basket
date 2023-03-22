use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use anyhow::Result;
use log::info;

const BUFFER_CAPACITY: u16 = std::u16::MAX;
const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

fn main() -> Result<()> {
    env_logger::init();

    let mut buffer = [0; BUFFER_CAPACITY as usize];
    let listener = TcpListener::bind((HOST, PORT)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            info!("New connection {:?}", stream);

            read_command_from_stream(&mut stream, &mut buffer)?;
        }
    }

    Ok(())
}

fn read_command_from_stream(stream: &mut TcpStream, buffer: &mut [u8]) -> std::io::Result<()> {
    let mut size_bytes = [0; 2];
    stream.read(&mut size_bytes)?;

    let len = u16::from_be_bytes(size_bytes);

    if len < BUFFER_CAPACITY {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer Overflow",
        ));
    }

    stream.read(buffer)?;

    Ok(())
}
