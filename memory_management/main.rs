// Rust has its own ownership model for memory management
// to avoid memory management issues such as dangling pointers/memory issues
// in c++, c, rust has some rules to handle them at compiler time.

// this makes it extremely safe against memory errors

// You can write code, access memory but you will have to follow some rules.

// not having a garbage collector is one of the key reasons rust is fast,
// it achieves this using
// mutability, heap and memory, ownership memory, lifetimes, borrowing and references

// Mutability
// these variables represent variables whose values can be changed once assigned

// Immutable -> values cant be changed
// In Rust, variables are by default immutable

// 1. Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently

// 2. Knowing that certain data will not change allows the compiler to optimize coder better.

// fn main() {
//     let x: i8 = 10;
//     println!("immutable x : {}", x);

//     let mut y: i8 = 20;
//     println!("mutable x : {}", y);

//     y = 30;
//     println!("mutable x : {}", y);
// }

// Stack => Fast allocation and deallocation. Rust uses stack for most primitive data types and for data where size is known at compile time

// Heap => Used for data that can grow at runtime, such as vectors or strings

// fn main() {
//     let s1: String = String::from("hello, world!!");
//     print!("{}", s1)
// }

fn main() {

    stack_fn();
    heap_fn();
    update_str();

}

fn stack_fn() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("Stack function => a + b : {}", c);
}

fn heap_fn() {
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let combined: String = format!("{}, {}!", s1, s2);
    println!("Heap function : combined string => {}", combined);
}

fn update_str() {
    let mut s: String = String::from("Initial string");
    println!("Before update: {}", s);
    println!("capacity: {}, length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str(" and some addition");
    println!("After update: {}", s);
    println!("capacity: {}, length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}