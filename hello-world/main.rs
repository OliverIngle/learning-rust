fn say_hi(foo: bool) {
    if foo {
        println!("hi")
    }
}



fn main() {
    println!("Hello, world!");
    say_hi(false)
}