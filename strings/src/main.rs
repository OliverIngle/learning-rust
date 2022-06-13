fn main() {

    let c = 'H';
    let mut s = String::new();
    s.push(c);
    s.push_str("llo");

    // for c in s.chars() {
    //     println!("{}", c);
    // }

    // for b in s.bytes() {
    //     println!("{}", b);
    // }

    for b in "v".bytes() {
        println!("{}", b)
    }

}
