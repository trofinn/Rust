use std::io::{Write,Read};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use rand::Rng;

fn serialize_and_send_message(mut stream : TcpStream, message : Message) {
    let serialized = serde_json::to_string(&message).unwrap();     
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()); 
    stream.write(serialized.as_bytes());
    println!("{:?}",serialized);
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message = Message::StartServer;
    serialize_and_send_message(stream,message);
}

#[derive(Serialize, Deserialize, Debug)]
enum Message { StartServer }