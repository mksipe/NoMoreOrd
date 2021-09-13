#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let anothermatch = App::from_yaml(yaml).get_matches();

    match matches {
        _ciphertext     => to_ascii(anothermatch.value_of("ciphertext").unwrap()),
    }
}
fn to_ascii(input:&str){
    use std::char;
    let input: u32 = input.parse::<u32>().unwrap();
    let out: Option<char> = char::from_u32(input);
    println!("{:?}",out.unwrap());
}