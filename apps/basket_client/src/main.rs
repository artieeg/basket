use basket_cli::serialize_command;
use basket_defs::{command::{Command, Action, Value}, command_result::CommandResult};
use std::{net::TcpStream, io::{Write, Read}};
use rkyv::{AlignedVec, Deserialize};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 3000;

fn main() -> std::io::Result<()> {
    let mut connection = TcpStream::connect((HOST, PORT)).unwrap();

    let command = Command {
        timestamp: 0,
        action: Action::SET(12321, Value::INT(42))
    };

    let bytes = serialize_command(&command);

    connection.write(&[0, bytes.len() as u8]).unwrap();
    connection.write(&bytes).unwrap();

    let mut response_len = [0; 2];
    connection.read(&mut response_len)?;

    let len = u16::from_be_bytes(response_len);

    let mut bytes = [0; std::u16::MAX as usize];
    connection.read(&mut bytes).unwrap();

    let mut vec = AlignedVec::new();
    vec.extend_from_slice(&bytes.split_at(len.into()).0);

    let archived = unsafe { rkyv::archived_root::<CommandResult>(&vec[..]) };
    let command_result: CommandResult = archived.deserialize(&mut rkyv::Infallible).unwrap();

    println!("COMMAND RESULT: {:#?}", command_result);

    Ok(())
}
