// What will happen when you try to compile and run the following code? Explain why.

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    // the owner has been changed to s2 and rust will delete s1 from stack
    // rust's ownership rule -> one value = one ownership
    // below code will work
    println!("{}", s2);

    // below code will not
    // println!("{}", s1);

    let s3 = String::from("World");
    let len = calculate_length(&s3);
    println!("length of {} is {}", s3, len);

    // Passing s1 by value to calculate_length moves ownership into the function.

    // After the call, s1 is no longer valid in main, so printing it causes a compile error.

    // What’s the difference between mutable and immutable references? Why does this code fail?

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &s;
    println!("{}", r1);

    // method 2
    let mut s4 = String::from("Hello, World!!");
    {
        let r1 = &s4;
        println!("{}", r1);
    }

    // It worked, because as the scope ends, r1 will go out of stack and s4 will not have any borrower
    let r2 = &mut s4;
    r2.push_str(" bye");
    println!("{}", r2);

    // This code will give error because it violates one scenario,
    // one owner can have multiple borrowers only if the borrowers are reading that value
    // If the borrower wants to modify the value, then it that value must have one borrower and not multiple

    // Limit the scope of your borrows by introducing inner blocks so that borrows don’t overlap.

    let s5 = String::from("Rust");
    let len = {
        let r = &s5;
        r.len()
    };
    println!("The length of '{}' is {}.", s5, len);

    let mut data = vec![1, 2, 3];
    let r3 = data[0];
    data.push(4);
    println!("r1 -> {}", r3);
    // println!("data -> {:?}", data);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("{}", r1);
    }
    let r4 = &mut s;
    r4.push_str("!");
    println!("{}", r4);

    // Rust allows only one mutable reference at a time—but here, r1’s scope ends before r2 is created.

    // The non-overlapping lifetimes ensure safety, so both borrows are valid sequentially.
}

// This prevents double free and dangling pointer bugs common in other languages.

// Inner blocks are a powerful way to manage borrow lifetimes without restructuring your entire function.

// Consider this function signature. Why does it not compile? How would you fix it?

fn calculate_length(s: &String) -> usize {
    s.len()
}