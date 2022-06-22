use std::char;

#[derive(Debug)]
pub struct Cipher {
    shift: u32,
    text: String,
}

impl Cipher {

    pub fn new(args: &[String]) -> Result<Cipher, &str> {

        if args.len() < 3 {
            return Err("Error: missing arguments")
        }

        let shift = args[1].clone();
        let shift: u32 = match shift
            .trim()
            .parse() {
                Err(_) => return Err("Error: shift must be a number (u32)"),
                Ok(num) => num
            };

        let text = args[2].clone();

        Ok(
            Cipher  { shift, text }
        )

    }

    pub fn apply(&self) -> Result<String, &str> {

        let mut ciphered = String::new();

        for c in self.text.chars() {
            match char::from_u32(c as u32 + self.shift) {
                Some(character) => {
                    ciphered.push(character);
                },
                None => return Err("Error: unable to cipher")
            }
        }

        Ok(ciphered)

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn shift_by_one() {

        use crate::Cipher;

        let foo = Cipher::new(&vec![
            String::new(),
            String::from("1"),
            String::from("abc"),
        ]).unwrap();
        assert_eq!(
            String::from("bcd"),
            foo.apply().unwrap()
        );
    }
}
