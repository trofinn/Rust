use std::io::prelude::*;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use rand::Rng;

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
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    answer : ChallengeAnswer,
    next_target : String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Welcome {
    version : u8
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

fn play_rounds(mut stream : &TcpStream) {
    
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
                    Message::RoundSummary(data) => { /*println!("{:?}\n",data)*/},
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

#[derive(Serialize, Deserialize, Debug)]
enum SubsribeResult{ Ok, Err(SubscribeError)}

#[derive(Serialize, Deserialize, Debug)]
enum SubscribeError{ AlreadyRegistered, InvalidName }