#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("ciphertext"){
        to_ascii(matches.value_of("ciphertext").unwrap());
    } else if matches.is_present("inputfile") {
        file_to_ascii(matches.value_of("inputfile").unwrap());
    } else {
        println!("{}", "Please input a valid argument.");
    }
}
pub fn to_ascii(input:&str<>) {
    use std::char;
    let input: u32 = input.parse::<u32>().unwrap();
    let out: Option<char> = char::from_u32(input);
    println!("{:?}",out.unwrap());
}


pub fn f_to_ascii(input:&str<>) -> Option<char> {
    use std::char;
    let input: u32 = input.parse::<u32>().unwrap();
    let out: Option<char> = char::from_u32(input);
    return out;
}

pub fn file_to_ascii(file_name:&str){
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(file_name).unwrap();
    let readfile = BufReader::new(file);
    for (i, o) in readfile.lines().enumerate() {
        let out = f_to_ascii(&o.unwrap());
        println!("line {}\t-> (\t{:?}\t)", i,out.unwrap());
    }

}