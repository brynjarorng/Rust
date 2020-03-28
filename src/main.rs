use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Welcome to this mandatory toothbrushing session :");
    println!("What is your message?");
  
    let stdout=stdout();
    let mut out = b"Hello there fellow rustaceans!";
//    let mut out = String::new();

    std::io::stdin().read_line(&mut out);

    let width: usize = out.len();//.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(out,width, &mut writer).unwrap();

    /*
    let mut _name2 = String::new();    
    std::io::stdin().read_line(&mut _name2);
    let x = 30;
    println!("Hello, {}!: {}", _name2, x);
    */


}