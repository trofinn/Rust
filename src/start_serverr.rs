use std::io;
use std::net::TcpStream;
mod md5hashcash;
mod connexions;
use crate::connexions::serialize_and_send_message;
use crate::md5hashcash::Message;

fn main() -> Result<(), io::Error>{
    let stream = TcpStream::connect("127.0.0.1:7878")?;
    let message = Message::StartServer;
    serialize_and_send_message(&stream,message)?;
    Ok(())
}