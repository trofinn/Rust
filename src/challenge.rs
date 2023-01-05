use rand::Rng;

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let seed : u64 = rng.gen();
        let message = "Hello";

        let hash = md5::compute(format!("{:016X}", seed) + &message);
        let md5 = format!("{:032X}", hash);
        println!("{:?}",md5);
        if check_hash(5,md5.clone()) {
            println!("yes");
            break;
        }
    }
    
    
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