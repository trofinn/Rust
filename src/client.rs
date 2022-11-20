use std::io::{self,prelude::*};
use std::net::TcpStream;



fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("failed");

        let len = message.len() as u32;
        stream.write(&len.to_le_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
        stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel*/

    }
    
}
