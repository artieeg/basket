mod store;
mod buffer;
mod command_reader;
mod parse_command;
mod serialize_command_result;

use crate::serialize_command_result::serialize_command_result;
use crate::command_reader::read_command_from_stream;
use crate::parse_command::parse_command;
use crate::store::Store;

use anyhow::Result;
use buffer::BUFFER_CAPACITY;
use log::info;
use std::{net::{TcpListener, TcpStream}, io::Write};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

fn main() -> Result<()> {
    env_logger::init();

    let mut buffer = [0; BUFFER_CAPACITY as usize];
    let listener = TcpListener::bind((HOST, PORT)).unwrap();
    let mut store = Store::new();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            info!("New connection {:?}", stream);

            let command_len = read_command_from_stream(&mut stream, &mut buffer)?;

            let command = parse_command(&buffer.split_at(command_len.into()).0)?;

            let result = store.apply(&command);

            let bytes = serialize_command_result(&result);

            stream.write(&[0, bytes.len() as u8])?;
            stream.write(&bytes)?;

            println!("Command {:#?}", command);
        }
    }

    Ok(())
}
