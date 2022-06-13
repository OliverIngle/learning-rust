fn main() {

    // Scalar types
    // Ints
    let y = 0b10100101;
    println!("y in binairy is {:b}", y);

    // Mutability, floats
    let mut x: f32 = 5.0;
    println!("x is {}", x);

    x = 0.5;
    println!("x is now {}", x);

    // Shadowing
    let x = x * 2.0;
    println!("x is now {}", x);

    // Constants
    const PI: f32 = 3.14;
    println!("Pi is {}", PI);

    // Strings
    let mut foo = "hello";
    println!("foo is {}", foo);

    foo = "hi";
    println!("foo is now {}", foo);

    // Chars
    let bar = 'h';
    println!("bar character is {}", bar);

    // Operators
    let division: f64 = 5. / 3.; // only works with floats
    println!("division is {}", division);

    //let sum = 1 + 3;
    //let diff = 3  - 5;
    //let prod = 5 * 4;
    //let floored = 5 / 8;
    //let remainder = 8 % 3;
    

    // Booleans
    let t = true;
    println!("t is {}", t);

    // Compound types
    // Tuples

    let tup: (f64, i32, u32) = (3.14, -42, 42);
    let tuup = ("hi", 96);

    println!("tup is {:?}", tup);
    println!("tuup is {:?}", tuup);
    println!("tuup's first element is {}", tuup.0);

    // Destructuring
    let (ay, bee, sea) = (1, 2, 3);
    println!("{}, {}, {}", ay, bee, sea);

    // Arrays
    let arr = [1, 42, 69, 420];
    let arrr: [char; 6] = ['o', 'l', 'i', 'v', 'e', 'r'];

    for i in arrr {
        println!("{}", i)
    }
    println!("second element of arr is {}", arr[1]);
}
