use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub name : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard(pub Vec<PublicPlayer>);

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64 ,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
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
    pub answer : ChallengeAnswer,
    pub next_target : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version : u8
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer{MD5HashCash(MD5HashCashOutput)}

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge{MD5HashCash(MD5HashCashInput)}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCash(pub MD5HashCashInput);

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name : String,
    pub value : ChallengeValue
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult(BadResult),
    Ok(Ok),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadResult {
    pub used_time : f64,
    pub next_target : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ok {
    pub used_time : f64,
    pub next_target : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    pub challenge : String,
    pub chain : Vec<ReportedChallengeResult>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    pub leader_board : PublicLeaderBoard
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeTimeout {
    pub message : String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubsribeResult{ Ok, Err(SubscribeError)}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError{ AlreadyRegistered, InvalidName }
