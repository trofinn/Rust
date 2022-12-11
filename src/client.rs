use std::io::{self,prelude::*};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};


fn main() {

    
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // HELLO TO THE SERVER
    let message = String::from("\"Hello\"");
    let len = message.len() as u32;
    
    stream.write(&len.to_be_bytes());
    stream.write(message.as_bytes());

    // WELCOME
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()); 
    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()); 
    let s = String::from_utf8_lossy(&buf); 
    println!("\n 1 : \n");
    print!("{:?}",s);

    // SUBSCRIBE PLAYER
    let subscribe = Subscribe { name : "TEST".to_string() };
    let message = Message::Subscribe(subscribe);
    let serialized = serde_json::to_string(&message).unwrap();     
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()); 
    stream.write(serialized.as_bytes());
    
    // SUBSCRIBE RESULT
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()); 
    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()); 
    print!("{:?}",buf);


    // ROUNDS :

    let mut count = 0;
    loop {
        if count == 10 {break};
        let mut buf_len = [0u8; 4];
        stream.read_exact(buf_len.as_mut()); 
        let len = u32::from_be_bytes(buf_len);
        let mut buf = vec![0u8; len as usize]; 
        stream.read_exact(buf.as_mut()); 
        let s = String::from_utf8_lossy(&buf); 
        let deserialized = serde_json::from_str::<Message>(&s);
        print!("\n{:?}\n",deserialized);


        match deserialized {
            Ok(data) => {
                match data {
                    Message::Challenge(data) => {
                        println!("\n4:\n");
                        println!("{:?}",data);
                        let challenge_answer = ChallengeAnswer::MD5HashCash(MD5HashCash::solve(&data));
                        let challenge_result = ChallengeResult {
                            answer : challenge_answer,
                            next_target : "TEST".to_string() };

                            // SENDING CHALLENGE RESULT
                            let message = Message::ChallengeResult(challenge_result);
                            let serialized = serde_json::to_string(&message).unwrap();     
                            let len = serialized.len() as u32;
                            stream.write(&len.to_be_bytes()); 
                            stream.write(serialized.as_bytes());
                            println!("\n5:\n");
                            println!("{:?}",serialized);
                    },
                    Message::PublicLeaderBoard(data) => { println!("{:?}\n",data);}
                    Message::RoundSummary(data) => { println!("{:?}\n",data)},
                    Message::EndOfGame(_) => {println!("{:?}\n",data)},
                    Message::Subscribe(_data) => {},
                    Message::ChallengeResult(_) => {},
                };
            }
            Err(_error) => { println!("TRY AGAIN");}
        };
        count+=1;
    }
    
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe {
    name : String,
}


#[derive(Serialize, Deserialize, Debug)]
struct PublicLeaderBoard(Vec<PublicPlayer>);


#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64 ,
}


#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Subscribe(Subscribe),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(MD5HashCash),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    answer : ChallengeAnswer,
    next_target : String,
}


#[derive(Serialize, Deserialize, Debug)]
enum ChallengeAnswer{MD5HashCash(MD5HashCashOutput)}

trait Challenge {

    type Input;
    type Output;
    fn name() -> String;
    fn new(input: Self::Input) -> Self;
    fn solve(&self) -> Self::Output;
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCash{}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;
    fn name() -> String { String::from("MD5HashCash")}
    fn new(input: Self::Input) -> Self {MD5HashCash {}}
    fn solve(&self) -> Self::Output {
        let output = MD5HashCashOutput { seed: (5), hashcode: (String::from("XXX")) };
            return output;
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