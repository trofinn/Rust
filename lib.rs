
trait Challenge {
    /// Donn�es en entr�e du challenge
    type Input;
    /// Donn�es en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// R�sout le challenge
    fn solve(&self) -> Self::Output;
    /// V�rifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}
#[derive(Debug)]
enum SubscribeError {
    AlreadyRegistered, 
    InvalidName
}
#[derive(Debug)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64 ,
}

enum ChallengeAnswer {
     ChallengeName(String),
}

struct ChallengeResult {
    name: ChallengeAnswer,
    next_target: String,
}

enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

pub trait Message {
    fn send(&self){
        println!("TO DO send method");
    }
    fn serialize(&self) {
        println!("TO DO serialize method");
    }
}
// BEFORE ROUND :
// INSCRIPTION PROTOCOL :


struct Welcome {
    version : u8,
}
#[derive(Serialize, Deserialize, Debug)]
struct Subscribe {
    name : String,
}
#[derive(Debug)]
enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

impl Message for Welcome {}
impl Message for Subscribe {}
impl Message for SubscribeResult{}

pub fn inscription(player : &PublicPlayer) {
    let subscribe_message = Subscribe { name : String::from("salut"),};
    subscribe_message.serialize();
    subscribe_message.send();
}

#[derive(Debug)]
struct PublicLeaderBoard(Vec<PublicPlayer>);

impl Message for PublicLeaderBoard {}


fn main() {
    let error = SubscribeError::InvalidName;
    match error {
        SubscribeError::InvalidName => println!("{:?}",error),
        _ => println!("non"),
    };
    
    let public_player = PublicPlayer {
        name : String::from("gesco"),
        stream_id: String::from("127.0.0.3"),
        score : 0,
        steps : 0,
        is_active : true,
        total_used_time : 0.0,
        };
    println!("{}, {}",public_player.name, public_player.stream_id);
    
    let _test = "salutare";
    let answer = ChallengeAnswer::ChallengeName("salutare".to_string());
    match answer {
        ChallengeAnswer::ChallengeName(_test) => println!("yes"),
        _ => println!("non"),
    };
    
    let bad_challenge_value = ChallengeValue::BadResult{ used_time : 0.0, next_target: "gesco".to_string()};
    match bad_challenge_value {
        ChallengeValue::BadResult{used_time, .. } => println!("used_time"),
        _ => println!("non"),
    };
    
    let ok_challenge_value = ChallengeValue::Ok{ used_time : 1.1, next_target : "gesco".to_string()};
    match ok_challenge_value {
        ChallengeValue::Ok{used_time, .. } => println!("{}", used_time),
        _ => println!("non"),
    };
    
    /*
    let subscribe_message = Subscribe { name : String::from("salut"),};
    subscribe_message.send();
    subscribe_message.serialize();*/
    
    inscription(&public_player);
    
    let mut players_vec : Vec<PublicPlayer> = Vec::new();  
    players_vec.push(public_player);
    let leaderboard = PublicLeaderBoard(players_vec);
    println!("{:?}",leaderboard);
    
    
    
 
}
///////////////////////

trait Challenge {
    /// Donn�es en entr�e du challenge
    type Input;
    /// Donn�es en sortie du challenge
    type Output;
    /// Nom du challengez
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// R�sout le challenge
    fn solve(&self) -> Self::Output;
    /// V�rifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}



struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

struct HashCash {}

impl Challenge for HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;
    fn name() -> String {
        return String::from("HashCash")
    }
    fn new(input : Self::Input) -> Self {
        // TO DO implement challenge
        // input.complexity convert from dec to string
        // 
        Self{}
    }
    fn solve(&self) -> Self::Output {
        //TO DO solve challenge
        //get a random seed with the good format
        // concat Self::Output.seed + Self::Input.message
        //md5 the value of concat and copy in hashcode
        let output = MD5HashCashOutput{ seed : 12, hashcode: "#932".to_string()};
        output
    }
    fn verify(&self, answer: &Self::Output) -> bool {
        //TO DO verify answer 
        // if Self::Output.hashcode has a nbr >= of 0 than complexity
        // return true;
        true
    }
}




fn main() {
    
}