

enum Coin {
    Penny,
    Nickel,
}

fn value_of_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
    }
}


fn main() {
    println!("{}", value_of_coin(Coin::Nickel))
}