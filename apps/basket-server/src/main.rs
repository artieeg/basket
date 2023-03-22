use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use anyhow::Result;
use bytecheck::CheckBytes;
use log::info;
use rkyv::{Archive, Deserialize, Serialize};

type BufferCapacity = u16;
const BUFFER_CAPACITY: BufferCapacity = BufferCapacity::MAX;
const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

type Buffer = [u8; BUFFER_CAPACITY as usize];

type Key = u32;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
enum Value {
    INT(i32),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
enum Action {
    SET(Key, Value),
    GET(Key),
    DEL(Key),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// This will generate a PartialEq impl between our unarchived and archived types
#[archive(compare(PartialEq))]
// To use the safe API, you have to derive CheckBytes for the archived type
#[archive_attr(derive(CheckBytes, Debug))]
struct Command {
    action: Action,
    timestamp: u128,
}

fn main() -> Result<()> {
    env_logger::init();

    let mut buffer = [0; BUFFER_CAPACITY as usize];
    let listener = TcpListener::bind((HOST, PORT)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            info!("New connection {:?}", stream);

            read_command_from_stream(&mut stream, &mut buffer)?;
            let command = parse_command(&buffer);

            println!("Command {:#?}", command);
        }
    }

    Ok(())
}

fn parse_command(buffer: &Buffer) -> Result<Command> {
    let archived = unsafe { rkyv::archived_root::<Command>(&buffer[..]) };

    let command: Command = archived.deserialize(&mut rkyv::Infallible)?;

    Ok(command)
}

fn read_command_from_stream(stream: &mut TcpStream, buffer: &mut Buffer) -> std::io::Result<()> {
    let mut size_bytes = [0; 2];
    stream.read(&mut size_bytes)?;

    let len = BufferCapacity::from_be_bytes(size_bytes);

    if len < BUFFER_CAPACITY {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer Overflow",
        ));
    }

    stream.read(buffer)?;

    Ok(())
}
