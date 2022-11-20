use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {

    loop {
        let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
        stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

        let len = u32::from_le_bytes(buf_len); // convertit les 4 octets en un entier u32

        let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
        stream.read_exact(buf.as_mut()).unwrap(); // on remplit le buffer

        let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
        println!("{s}"); // en String
    }
    
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
