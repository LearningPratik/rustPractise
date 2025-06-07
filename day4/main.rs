fn main() {
    let a: i32 = 10;
    let b: i32 = 100;
    let sum: i32 = do_sum(a, b);
    println!("Sum of {} and {} : {}", a, b, sum)
}


fn do_sum(a: i32, b: i32) -> i32 {
    return a + b
}



fn hello() {
    println!("Hello, World!!")
}


// Print numbers using a for loop

fn print_nums() {
    for i in 1..=6 {
        println!("{}", i)
    }
}

// Write a Rust program that checks if a number is even or odd.

fn even_odd() {
    let num: i8 = 8;

    if num % 2 == 0 {
        println!("Even number")
    } else {
        println!("Odd number")
    }
}


// Write a Rust program that iterates over an array and prints each element.
// let cannot be used for global variables
// // use static or const for global variables

const NUMS : [i32; 5] = [1, 2, 3, 4, 5];
fn print_array() {

    for num in NUMS.iter() {
        println!("{}", num)
    }
}

// fn main() {

//     let numbers = [1, 2, 3, 4, 5];

//     // for i in numbers.iter() {
//     //     println!("{}", i)
//     // }

//     for i in 0..numbers.len() {
//         println!("{} number at {} index", i, numbers[i])
//     }
// }

// Write a Rust program that uses a while loop to count down from 10 to 1.

fn print_array_reverse() {

    let mut i = 10;
    while i >= 0 {
        println!("{}", i);
        i = i - 1;
    }
}