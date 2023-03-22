mod store;
mod buffer;
mod command_reader;
mod parse_command;

use crate::command_reader::read_command_from_stream;
use crate::parse_command::parse_command;
use crate::store::Store;

use anyhow::Result;
use buffer::BUFFER_CAPACITY;
use log::info;
use std::net::{TcpListener, TcpStream};

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

            let _result = store.apply(&command);

            println!("Command {:#?}", command);
        }
    }

    Ok(())
}
