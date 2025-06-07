// one owner but multiple borrowers
// I bought a car (im the owner)
// and i give my car as rent to other users (they are borrowers)

// here only I can use the car, however I want to
// but borrowers can not do anything, they have to follow rules.

// 3 scenarios here
// 1. Only one owner 
//    one value = one owner

// 2. One owner and multiple borrowers
//    here, when multiple borrowers then strict rules will apply

// 3. One owner and one borrower
//    here, no rules for both
//    Imagine, this car is rented to your close relative, you can put rules here so, they will use it like you use it.

// Value will only gets deleted if the owner gets deleted.


// References => giving the address of a string rather than the ownership of the string to a function

// fn main() {
//     let s1 = String::from("Will be borrowed");
//     let s2 = &s1;

//     println!("{}", s1);
//     println!("{}", s2);
// }

// fn main() {
//     let my_str: String = String::from("borrowed");
//     borrow_variable(&my_str);
//     println!("{}", my_str);

// }

// fn borrow_variable(some_str: &String) {
//     println!("{}", some_str);
// }

// Mutable reference
// fn main() {
//     let mut s1 = String::from("Hello");
//     update_str(&mut s1);
//     println!("{}", s1);
// }

// fn update_str(some_str: &mut String) {
//     some_str.push_str(" World!!");
// }

// This is the example of 3rd scenario
// 3. One owner and one borrower
//    here, no rules for both
//    Imagine, this car is rented to your close relative, you can put rules here so, they will use it like you use it.

fn main() {
    let mut my_str = String::from("Hello");
    let some_str = &mut my_str;
    some_str.push_str(" world!!!!!!");

    println!("{}", some_str);
    println!("{}", my_str);
}