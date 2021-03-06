use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("> Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..51);

    loop {
        println!("> Please input your guess:");
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(input) => println!("'{}' was inputted", input),
            Err(_) => println!("Error: failed to read input")
        }

        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error: Input must be a number.");
                    continue;
                }
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("The secret number was: {}", secret_number);
                println!("You win!");
                break;
            }
        }
    }
}
