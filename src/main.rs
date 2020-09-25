use lexpr::{self, parse::Options};
use std::fs;

fn main() {
    println!("reading test pcb...");

    let contents =
        fs::read_to_string("ferret.kicad_pcb").expect("Something went wrong reading the file");

    let p = "(kicad_pcb";

    let t = if contents.starts_with(p) {
        &contents[p.len()..contents.len() - 3]
    } else {
        &contents
    };

    let options = Options::kicad();    
    let results = lexpr::from_str_custom(&contents, options);

    println!("{}", results.unwrap())
}
