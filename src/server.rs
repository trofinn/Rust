use std::io::{Write,Read};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};


fn handle_client(mut stream: TcpStream) {


    // RECEIVING HELLO FROM THE CLIENT
    let mut buf_len = [0u8; 4]; 
    stream.read_exact(buf_len.as_mut()).unwrap(); 
    let len = u32::from_le_bytes(buf_len); 
    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()).unwrap(); 
    let s = String::from_utf8_lossy(&buf); 
    println!("{s}"); 

    /* 
    let serialized = String::from("");
    let deserialized = serde_json::from_str::<Message>(&serialized);
    match deserialized {
        Ok(data) => {println!("{:?}", data) }
        Err(error) => { println!("TRY AGAIN");}
    }*/

    // SENDING WELCOME TO THE CLIENT
    
    match s {
        std::borrow::Cow::Borrowed("Hello") => { 
            let message = String::from("Welcome!");
            let len = message.len() as u32;
            stream.write(&len.to_le_bytes()); 
            stream.write(message.as_bytes()); 
        },
        std::borrow::Cow::Owned(_) => todo!(),
        std::borrow::Cow::Borrowed(_) => todo!(),
    };


    

}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_client(stream),
                    Err(error) => panic!("Problem looping through the connexions: {:?}", error)
                };
            }
        },
        Err(error) => panic!("Problem creating the bind: {:?}",error)
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe {
    name : String,
}
#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Subscribe(Subscribe)
}