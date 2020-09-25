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
            let name = sym.to_string();
            match name.as_str() {
                "version" => println!("version {:#?}", v[1]),
                "general" => println!("general {:#?}", v[1]),
                "page" => println!("page {:#?}", v[1]),
                "layers" => println!("layers {:#?}", v[1]),
                "setup" => println!("setup {:#?}", v[1]),
                "net" => println!("net {:#?}", v[1]),
                "net_class" => println!("net_class {:#?}", v[1]),
                "module" => println!("module {:#?}", v[1]),
                "segment" => println!("segment {:#?}", v[1]),
                "via" => println!("via {:#?}", v[1]),
                "dimension" => println!("dimension {:#?}", v[1]),
                "gr_circle" => println!("gr_circle {:#?}", v[1]),
                "gr_text" => println!("gr_text {:#?}", v[1]),
                "gr_line" => println!("gr_line {:#?}", v[1]),
                "gr_arc" => println!("gr_arc {:#?}", v[1]),
                "zone" => println!("zone {:#?}", v[1]),
                _ => println!("uwu, what is this? {}", name),
            }
        } else {
            println!("{:#?}", v.to_vec());
        }
    }
}
