mod md5hashcash;
mod md5implementation;
mod connexions;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::connexions::*;
use crate::md5hashcash::*;


fn handle_inscription(stream : &TcpStream, mut public_leaderboard : PublicLeaderBoard) -> Result<PublicLeaderBoard, io::Error> {
    
    loop {
        let deserialized = serde_json::from_str::<Message>(&read_message(&stream))?;
        match deserialized {
            received_message => {
                match received_message {
                    Message::Hello => {
                        let welcome = Welcome { version : 1};
                        let message = Message::Welcome(welcome);
                        serialize_and_send_message(stream, message);
                    }
                    Message::Subscribe(received_message) => {
                        let public_player = PublicPlayer {
                            name : received_message.name.clone(),
                            stream_id : "000".to_string(),
                            score : 0,
                            steps : 0,
                            is_active : true,
                            total_used_time : 0.0
                        };
                        let subscribe_result: SubsribeResult;
                        if public_leaderboard.0.is_empty() {
                            public_leaderboard.0.push(public_player);
                            subscribe_result = SubsribeResult::Ok;
                        }
                        else {
                            if public_leaderboard.0.contains(&public_player) {
                                subscribe_result = SubsribeResult::Err(SubscribeError::AlreadyRegistered);
                                println!("A player with the same name already exists!\n");
                                
                            }
                            else {
                                public_leaderboard.0.push(public_player);
                                subscribe_result = SubsribeResult::Ok;
                            }
                        }
                        let message = Message::SubscribeResult(subscribe_result);
                        serialize_and_send_message(stream, message);
                        println!("{:?}",&public_leaderboard);
                    }
                    Message::Challenge(_) => {},
                    Message::ChallengeTimeout(_) => {},
                    Message::Welcome(_) => {},
                    Message::PublicLeaderBoard(_) => {},
                    Message::RoundSummary(_) => {},
                    Message::SubscribeResult(_) => {},
                    Message::ChallengeResult(_) => {},
                    Message::EndOfGame(_) => {}
                    Message::StartServer => {
                        println!("Start signal received\n");
                        return Ok(public_leaderboard)
                    }
                }
            }
        }
    }
}


fn handle_client(stream: &TcpStream) -> Result<(), io::Error>{

    
    // RECEIVING HELLO FROM THE CLIENT
    let public_players : Vec<PublicPlayer>  = vec![];
    let mut public_leaderboard = PublicLeaderBoard(public_players);
    public_leaderboard = handle_inscription(stream,public_leaderboard)?;
    println!("aaa{:?}",public_leaderboard);
    Ok(())  
}

fn main() -> Result<(), io::Error>{
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        handle_client(&stream?);
    }
    Ok(())
}