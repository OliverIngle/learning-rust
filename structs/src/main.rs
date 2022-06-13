
#[derive(Debug)]
struct Diary {
    name: String,
    text: String
}

impl Diary {

    fn new(name: String) -> Diary {
        Diary {
            name,
            text: String::new()
        }
    }

    fn write(&mut self, text_to_write: &String) {
        self.text.push_str(&text_to_write[..]);
    }
}


fn main() {

    let mut my_diary = Diary::new(
        String::from("Oliver's diary")
    );

    my_diary.write(
        &String::from("Today i learnt more about rust!")
    );
    println!("{:#?}", my_diary);

    my_diary.write(
        &String::from(" I learnt about structs and enums.")
    );
    println!("{:#?}", my_diary);

}