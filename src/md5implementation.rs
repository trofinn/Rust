use rand::Rng;

use crate::md5hashcash::*;

pub trait Challengee {

    type Input;
    type Output;
    fn name() -> String;
    fn new(input: Self::Input) -> Self;
    fn solve(&self) -> Self::Output;
    fn verify(&self, answer: &Self::Output) -> bool;
}

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
    fn verify(&self, _answer: &Self::Output) -> bool {false}
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