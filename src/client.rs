use std::net::TcpStream;

mod md5hashcash;
mod md5implementation;
mod connexions;
use crate::connexions::*;
use crate::md5hashcash::*;
use crate::md5implementation::*;

fn main() {

    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    inscription(&stream, String::from("Test"));
    
    // ROUNDS :

    let mut count = 0;
    
    loop 
    {
        if count == 15 {break};
        play_rounds(&stream);
        count+=1;
    }
}

 

fn inscription(stream : &TcpStream, _name : String) {

    // HELLO TO THE SERVER

    let message = Message::Hello;

    serialize_and_send_message(stream, message);

    // WELCOME
    read_message(stream);

    // SUBSCRIBE PLAYER
    let subscribe = Subscribe { name : "TEST".to_string() };
    let message = Message::Subscribe(subscribe);
    serialize_and_send_message(stream, message);
    
    
    // SUBSCRIBE RESULT
    read_message(stream);
}




fn play_rounds(stream : &TcpStream) {
    
        let deserialized = serde_json::from_str::<Message>(&read_message(&stream));
        print!("\n{:?}\n",deserialized);
        let mut next_target = String::from("");
        match deserialized {
            Ok(data) => {
                match data {
                    Message::Challenge(data) => {
                        match data {
                            Challenge::MD5HashCash(challenge) => {
                                let md5hashcash = MD5HashCash::new(challenge);
                                let output = MD5HashCash::solve(&md5hashcash);
                                let challenge_answer = ChallengeAnswer::MD5HashCash(output);
                                let challenge_result = ChallengeResult {
                                    answer : challenge_answer,
                                    next_target : next_target.clone(),
                                };
                                let message = Message::ChallengeResult(challenge_result);
                                serialize_and_send_message(&stream, message);
                            }
                        }
                    },
                    Message::PublicLeaderBoard(data) => { 
                        println!("{:?}\n",&data);
                        let mut min_score = 999;
                        for player in &data.0 {
                            if player.score < min_score {
                                min_score = player.score;
                            }
                        }
                        for player in data.0 {
                            if player.score == min_score {
                                next_target = player.name;
                            }
                        }
                    }
                    Message::RoundSummary(_data) => { /*println!("{:?}\n",data)*/},
                    Message::EndOfGame(_) => {println!("{:?}\n",data)},
                    Message::Subscribe(_data) => {},
                    Message::ChallengeResult(_) => {},
                    Message::ChallengeTimeout(_) => println!("TIMEOUT\n"),
                    Message::Hello => {},
                    Message::Welcome(_) => {},
                    Message::SubscribeResult(_) => {},
                    Message::StartServer => {}
                };
            }
            Err(_error) => { println!("TRY AGAIN");}
        };
}