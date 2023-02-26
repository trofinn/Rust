use std::{net::TcpStream, io::{Write, Read}};
use crate::md5hashcash::Message;

pub fn serialize_and_send_message(mut stream : &TcpStream, message : Message) {
    let serialized = serde_json::to_string(&message).unwrap();     
    let len = serialized.len() as u32;
    let test = stream.write(&len.to_be_bytes()); 
    stream.write(serialized.as_bytes());
    println!("{:?}",serialized);
}

pub fn read_message(mut stream : &TcpStream) -> String{
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()); 
    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()); 
    let s = String::from_utf8_lossy(&buf);
    println!("{:?}",&s);
    return s.to_string();
}