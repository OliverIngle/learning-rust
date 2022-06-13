use std::io;

fn main() {

    println!("{}", 97 as char);

    // println!("Text to be ciphered:");
    // let mut text = String::new();
    // match io::stdin().read_line(&mut text) {
    //     Ok(_) => println!("Success"),
    //     Err(_) => println!("Error: failed to read input")
    // }

    // println!("Shift [0, 255]:");
    // let mut shift = String::new();
    // match io::stdin().read_line(&mut shift) {
    //     Ok(_) => println!("Success"),
    //     Err(_) => println!("Error: failed to read input")
    // }

    // let text: String = text
    //     .trim()
    //     .to_string();

    // let shift: u8 =shift
    //     .trim()
    //     .parse()
    //     .expect("Error: failed to convert input to u8");

    // let result = cipher(&text, shift);
    // println!("Result: {}", result)

}

fn cipher(text: &String, shift: u8) -> String {

    let bytes: &[u8] = text.as_bytes();
    let mut new_bytes: Vec<u8> = vec![];

    for i in 0..bytes.len() {
        new_bytes.push(bytes[i] + shift)
    }

    match String::from_utf8(new_bytes) {
        Ok(txt) => txt,
        Err(_) => {
            println!("Error: Cipher failed");
            String::from("Cipher Failed")
        }
    }

}