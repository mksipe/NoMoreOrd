#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let anothermatch = App::from_yaml(yaml).get_matches();
    match matches {
        inputfile   => file_to_ascii(anothermatch.value_of("inputfile").unwrap()),
        ciphertext  => to_ascii(anothermatch.value_of("ciphertext").unwrap()),
    }
}
fn to_ascii(input:&str<>) {
    use std::char;
    let input: u32 = input.parse::<u32>().unwrap();
    let out: Option<char> = char::from_u32(input);
    println!("{:?}",out.unwrap());
}


fn f_to_ascii(input:&str<>) -> Option<char> {
    use std::char;
    let input: u32 = input.parse::<u32>().unwrap();
    let out: Option<char> = char::from_u32(input);
    return out;
}

fn file_to_ascii(file_name:&str){
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(file_name).unwrap();
    let readfile = BufReader::new(file);
    for (i, o) in readfile.lines().enumerate() {
        let out = f_to_ascii(&o.unwrap());
        println!("line {}\t-> (\t{:?}\t)", i,out.unwrap());
    }

}