fn borrows_array(arr: &mut [i32]) {
    println!("{:?}", arr)
}

//fn takes_array(arr: [i32; 2]) {}

fn takes_ownership(s: String) {}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn borrow_and_muts(arr: &mut [i32]) {
    println!("{}", arr[1]);

    arr[0] = 5;
}


fn main() {

    let a = [String::new()];
    let a1 = a;
    //println!("{:?}", a) //impossible
    let a = [1, 8, -9];
    let a1 = a;
    println!("{:?}", a); //possible

    let mut a: [i32; 4] = [1, 5, 9, -4];
    borrow_and_muts(&mut a);
    println!("{:?}", a)


    // let mystr = String::from("Hello");
    // takes_ownership(mystr);
    // let s2 = takes_and_gives_back(String::from("hey"));

    // let mut arr1: [i32; 2] = [6, 7];
    // arr1[0] = -8;
    // println!("{}", arr1[0]);
    // borrows_array(&arr1);

    // let s1 = String::from("Hello world");
    // let w1 = first_word(&s1);
    // println!("{}", w1);    
}


// fn first_word(s: &String) -> &str {
//     let arr = s.as_bytes();

//     for (i, &item) in arr.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }
