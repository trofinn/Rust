use serde::{Serialize, Deserialize};
use serde::de::Error;

#[derive(Serialize, Deserialize, Debug)]
struct MyStructure {
    size: u32,
    datatype: String,
    value: String,
}

impl MyStructure {
    fn to_string(&self) -> String {
        format!("{{ size: {}, datatype: \"{}\", value: \"{}\" }}", self.size, self.datatype, self.value)
        // format!("size: {s}, datatype: {d}, value: {v}", 
        //         s = self.size, 
        //         d = self.datatype, 
        //         v = self.value)
    }

    // s: String: argument couteux
    // fn from_str(s: String) -> MyStructure {
    //     todo!() // bcp de boulot
    // }

    // s: &str: argument léger
    // fn from_str(s: &str) -> MyStructure {
    //     todo!() // bcp de boulot
    // }
}

#[test]
fn main() {
    let n: i32 = -1;
    // let n2 = 0x00 00 00 01;
    // let n2 = 0x01 00 00 00;

    // for b in n.to_le_bytes() {
    //     println!("{b:0x}");
    // }
    // println!("----");
    // for b in n.to_be_bytes() {
    //     println!("{b:0x}");
    // }
    // println!("----");
    // for b in n.to_ne_bytes() {
    //     println!("{b:0x}");
    // }

    let s = 1.to_string();

    let mut message = "Hello";
    message = "Bye bye";
    let message = String::from("Hello");
    let mut message = "Hello".to_string();
    message.push('!');

    let s = MyStructure {
        size: 0,
        datatype: "str".to_string(),
        value: "hello".to_string(),
    };

    println!("{s:?}");

    let s1 = s.to_string();
    let s2 = s.to_string();
    println!("{s1}");

    assert_eq!(s1, r#"{ size: 0, datatype: "str", value: "hello" }"#);

    let serialized = serde_json::to_string(&s);
    match serialized {
        Ok(data) => { println!("{:}", data) }
        Err(error) => { panic!("CA MARCHE PAS : {:?}", error) }
    }

    let serialized = r#"{ size: 0, datatype: "str", value: "hello" }"#;

    let deserialized = serde_json::from_str::<MyStructure>(&serialized);
    match deserialized {
        Ok(data) => {println!("{:?}", data) }
        Err(error) => { println!("TRY AGAIN");}
    }

    // let f = n as u32;
    // println!("{f}");
}
