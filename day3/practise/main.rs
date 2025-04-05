// Print numbers using a for loop

fn main() {
    // 1..6 -> this will exclude 6
    // 1..=6 -> this will include 6
    for i in 1..=6 {
        print!("{} ", i)
    }
}

// Write a Rust program that checks if a number is even or odd.

fn main() {
    let num: i8 = 8;
    if num % 2 == 0 {
        println!("{} is a even number", num);
    } else {
        println!("{} is a odd number", num);
    }
}

// Write a Rust program that iterates over an array and prints each element.

fn main() {
    let number = [1, 2, 3, 4, 5];
    for num in number.iter() {
        print!("{} ", num);
    }
}

// Write a Rust program that uses a while loop to count down from 10 to 1.

fn main() {
    let mut i = 10;
    while i > 0 {
        println!("count down : {}", i);
        i = i - 1;
    }
    println!("Blast..")
}

// Write a Rust program that uses an infinite loop and breaks out when a condition is met.

fn main() {
    let mut j = 0;
    loop {
        j = j + 1;
        if j == 5 {
            println!("{} ", j);
            break;
        }
    }
}

// write multiplication table

fn main() {
    
    let n = 2;
    for i in 1..=10 {
        println!("{} * {} = {}", n, i, n*i)
    }
}

// Write a Rust program that categorizes a number into positive, negative, or zero using multiple conditions.

fn main() {
    let number: i8 = 0;

    if number > 0 {
        println!("{} is a positive number ", number)
    } else if number < 0 {
        println!("{} is a negative number ", number)
    } else {
        println!("zero..")
    }
}

// Write a Rust program that uses the continue keyword to skip printing the number 3 in a loop from 1 to 5.

fn main() {

    for i in 1..=5 {
        if i == 3 {
            continue
        }

        println!("{}", i)
    }
}

// Write a Rust program that iterates over an array of integers and prints only the even numbers.

fn main() {

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in array {
        if i % 2 == 0 {
            println!("{} is even number", i)
        }
    }
}