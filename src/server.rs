use std::io::{Write,Read};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use rand::Rng;
use std::sync::Mutex;
use std::sync::Arc;
use rand::prelude::IteratorRandom;

fn handle_inscription(mut stream : &TcpStream, mut public_leaderboard : PublicLeaderBoard) -> PublicLeaderBoard {
    
    loop {
        let deserialized = serde_json::from_str::<Message>(&read_message(&stream));
        match deserialized {
            Ok(received_message) => {
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
                        return public_leaderboard
                    }
                }
            }
            Err(err) => { panic!("error deserializing the message\n");}
        }
    }
    
}


fn handle_client(mut stream: &TcpStream) {

    
    // RECEIVING HELLO FROM THE CLIENT
    let mut public_players : Vec<PublicPlayer>  = vec![];
    let mut public_leaderboard = PublicLeaderBoard(public_players);
    public_leaderboard = handle_inscription(stream,public_leaderboard);
    println!("aaa{:?}",public_leaderboard);    
}



#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubsribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
    ChallengeTimeout(ChallengeTimeout),
    StartServer
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_client(&stream),
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
enum SubsribeResult{ Ok, Err(SubscribeError)}

#[derive(Serialize, Deserialize, Debug)]
enum SubscribeError{ AlreadyRegistered, InvalidName }

#[derive(Serialize, Deserialize, Debug)]
struct PublicLeaderBoard(Vec<PublicPlayer>);


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64 ,
}



#[derive(Serialize, Deserialize, Debug)]
struct Welcome {
    version : u8
}




#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    answer : ChallengeAnswer,
    next_target : String,
}


#[derive(Serialize, Deserialize, Debug)]
enum ChallengeAnswer{MD5HashCash(MD5HashCashOutput)}


#[derive(Serialize, Deserialize, Debug)]
enum Challenge{MD5HashCash(MD5HashCashInput)}
trait Challengee {

    type Input;
    type Output;
    fn name() -> String;
    fn new(input: Self::Input) -> Self;
    fn solve(&self) -> Self::Output;
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCash(MD5HashCashInput);

impl Challengee for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;
    fn name() -> String { String::from("MD5HashCash")}
    fn new(input: Self::Input) -> Self {MD5HashCash(MD5HashCashInput { complexity: (input.complexity), message: (input.message) })}

    fn solve(&self) -> Self::Output {

        loop 
        {
            let mut rng = rand::thread_rng();
            let seed : u64 = rng.gen();
    
            let hash = md5::compute(format!("{:016X}", seed) + &self.0.message);
            let md5 = format!("{:032X}", hash);
            if check_hash(self.0.complexity,md5.clone()) 
            {
                let output = MD5HashCashOutput{
                    seed : seed,
                    hashcode : md5,
                };
                return output;
            }
        }
        
    } 
    fn verify(&self, answer: &Self::Output) -> bool {false}
}

#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ReportedChallengeResult {
    name : String,
    value : ChallengeValue
}


#[derive(Serialize, Deserialize, Debug)]
enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult(BadResult),
    Ok(Ok),
}


#[derive(Serialize, Deserialize, Debug)]
struct BadResult {
    used_time : f64,
    next_target : String
}

#[derive(Serialize, Deserialize, Debug)]
struct Ok {
    used_time : f64,
    next_target : String
}


#[derive(Serialize, Deserialize, Debug)]
struct RoundSummary {
    challenge : String,
    chain : Vec<ReportedChallengeResult>
}

#[derive(Serialize, Deserialize, Debug)]
struct EndOfGame {
    leader_board : PublicLeaderBoard
}

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeTimeout {
    message : String
}

pub fn check_hash(mut complexity: u32, hash: String) -> bool {
    let bit_compare = 1 << 127;
    let mut sum = u128::from_str_radix(&*hash, 16).unwrap();
    while complexity > 0 {
        if (sum & bit_compare) > 0 {
            break;
        }
        sum = sum << 1;
        complexity -= 1;
    }
    complexity == 0
  }

  impl MD5HashCashInput {
    pub fn new() -> MD5HashCashInput {
        MD5HashCashInput { complexity: (3), message: (String::from("")) }
    }
  }


  fn serialize_and_send_message(mut stream : &TcpStream, message : Message) {
    let serialized = serde_json::to_string(&message).unwrap();     
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()); 
    stream.write(serialized.as_bytes());
    println!("{:?}",serialized);
}

fn inscription(mut stream : &TcpStream, name : String) {
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


fn read_message(mut stream : &TcpStream) -> String{
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()); 
    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()); 
    let s = String::from_utf8_lossy(&buf);
    println!("{:?}",&s);
    return s.to_string();
}