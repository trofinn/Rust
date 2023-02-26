use std::{net::TcpStream, io::{Write, Read}};
use serde_json::Error;

use crate::md5hashcash::Message;

pub fn serialize_and_send_message(mut stream : &TcpStream, message : Message) -> Result<(), Error>{
    let serialized = serde_json::to_string(&message)?;     
    let len = serialized.len() as u32;
    let _message_bytes = stream.write(&len.to_be_bytes()); 
    let _message_write = stream.write(serialized.as_bytes());
    println!("{:?}",serialized);
    Ok(())
}

pub fn read_message(mut stream : &TcpStream) -> String{
    let mut buf_len = [0u8; 4];
    let _read = stream.read_exact(buf_len.as_mut()); 
    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize]; 
    let _read = stream.read_exact(buf.as_mut()); 
    let s = String::from_utf8_lossy(&buf);
    println!("{:?}",&s);
    return s.to_string();
}