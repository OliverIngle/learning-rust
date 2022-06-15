use std::io;
use std::char;

fn main() {

    

    println!("Text to be ciphered:");
    let mut text = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => println!("Success"),
        Err(_) => println!("Error: failed to read input")
    }

    println!("Shift [u32 number]:");
    let mut shift = String::new();
    match io::stdin().read_line(&mut shift) {
        Ok(_) => println!("Success"),
        Err(_) => println!("Error: failed to read input")
    }

    let text: String = text
        .trim()
        .to_string();

    let shift: u32 = shift
        .trim()
        .parse()
        .expect("Error: failed to convert input to u8");

    let result = cipher(&text, shift);
    println!("Result: {}", result)


}

fn cipher(text: &String, shift: u32) -> String {

    let mut ciphered = String::new();

    for c in text.chars() {
        match char::from_u32(c as u32 + shift) {
            Some(character) => {
                println!("{} -> {}", c, character);
                ciphered.push(character);
            },
            None => {
                println!("Unable to shift character '{}', so kept as original. (shift probably exceded highest unicode value)", c);
                ciphered.push(c);
            }
        }
    }

    ciphered

}