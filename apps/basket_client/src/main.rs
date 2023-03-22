use basket_cli::serialize_command;
use basket_defs::command::{Command, Action, Value};
use std::{net::TcpStream, io::Write};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 3000;

fn main() {
    let mut connection = TcpStream::connect((HOST, PORT)).unwrap();

    let command = Command {
        timestamp: 0,
        action: Action::SET(12321, Value::INT(42))
    };

    let bytes = serialize_command(&command);
    println!("{:#?}", bytes);

    connection.write(&[0, bytes.len() as u8]).unwrap();
    connection.write(&bytes).unwrap();
}
