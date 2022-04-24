fn main() {

    // If/Else
    let number = 12;
    if  number % 4 == 0 {
        println!("mod 4");
    } else if number % 3 == 0 {
        println!("mod 3")
    } else {
        println!("none")
    }

    // If/Else in a let statement
    let condition = true;
    let is_true = if condition { "yes" } else { "No" };
    println!("{}", is_true);

    //Loops
    // FizzBuzz with for loop
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i)
        }
    }

    // loop
    let mut count = 0;
    loop {
        println!("Count is {}", count);
        if count == 10 {
            break;
        }
        count += 1;
    }

    // loop in loop, loop label
    'outer_loop: loop {
        println!("count is {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining is {}", remaining);
            if remaining == 5 { break; }
            if count == 15 { break 'outer_loop; }
            remaining -= 1;
        }

        count += 1;
    }

    // Ruturning from loops
    let result = loop {
        count += 1;
        if count == 20 {
            break count * 2;
        }
    };
    println!("Result is {}", result);

    // while loops
    while count != 25 {
        println!("count is {}", count);
        count += 1;
    }

    // for loop with reversed range
    for i in (1..6).rev() {
        println!("{}", i);
    }
    println!("Blastoff!!")

}
