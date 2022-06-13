use std::io;

fn main() {

    let mut number = String::new();

    println!("> Please input a number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Error: failed to get input.");

    let number: i32 = number
        .trim()
        .parse()
        .expect("Error: input wasnt a number (interger).");

    let result = add_one(number);

    println!("{} + 1 = {}", number, result)

    //my_func();

    // Statements vs expressions
    // Statements do something but dont return a value
    // let xyx = 123;
    // Expressions do return a value
    // let x = {
    //      let y = 1;
    //      y + 9  
    // };

}

//fn my_func() {
//   println!("my_func was called!");
//}


fn add_one (x: i32) -> i32 {
    x + 1
}