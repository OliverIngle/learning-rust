use std::env;
use caesar_cipher::Cipher;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    let cipher = Cipher::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);    
    });
    
    let cipher_text = cipher.apply().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);    
    });

    println!("{:?}", cipher_text);
}

