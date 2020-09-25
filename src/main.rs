use lexpr::{self, parse::Options};
use std::fs;

use serde_lexpr::{from_str_custom, to_string};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Version {
  name: String,
  age: u8,
}


fn main() {
    println!("reading test pcb...");

    let tuple: (u8, String, u64);

    let contents =
        fs::read_to_string("ferret.kicad_pcb").expect("Something went wrong reading the file");

    let p = "(kicad_pcb";

    let t = if contents.starts_with(p) {
        &contents[p.len()..contents.len() - 3]
    } else {
        &contents
    };

    let results = lexpr::from_str_custom(&contents, Options::kicad()).unwrap();

    // the pcb structure
    let pcb = results.as_pair().unwrap();
    let mut iter = pcb.1.list_iter().unwrap();
/*
    let version = iter.next().unwrap();
    let host = iter.next().unwrap();
    let page = iter.next().unwrap();
    let layers = iter.next().unwrap();
    let setup = iter.next().unwrap();
*/
    for value in iter {
        let v = value.to_vec().unwrap();
        let sym = v.first().unwrap();

        if !sym.is_cons() {
            match sym.to_string().as_str() {
                "version" => println!("{:#?}", v[1])
                
            }
        } else {
            println!("{:#?}", v.to_vec());
        }
    }
}
