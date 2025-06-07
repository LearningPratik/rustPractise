// Declare a mutable variable, assign a value, then change it

// i => signed (it can take negative and positive values)
// u => unsigned (it can only take positive values)
// by default variables are immutable in rust so, we need to use mut keyword to change the variable later

fn main() {
    let mut x: i8 = 8;
    println!("Before changing x => {}", x);

    x = -8;
    println!("After changing x => {}", x);

    let c = do_sum(8, 10);
    println!("sum of 8 and 10 => {}", c);

    let p = Point {x: 3.1, y: 3.2};
    println!("Point on fields x => {} and y => {}", p.x, p.y);

    println!("Area of rectangle : {}", p.area());

    let arr = [1, 2, 3, 4, 5];
    iterate_array(arr);

    iterate_array_2(arr);

    let my_str: String = String::from("Hello");
    takes_ownership(my_str);

    // Borrowing
    let my_str1: String = String::from("Hello, World!!");
    let some_str1 = &my_str1;

    println!("owner of the value, my_str1 : {}", my_str1);
    println!("taking reference of the value, some_str1 : {}", some_str1);

    let s1: String = String::from("Second example of reference");
    let s2: usize = calculate_length(&s1);
    println!("reference example with function, owner => {} and borrower => {}", s1, s2);
    
    // Here, the ownership is transferred to some_str (passed in takes_ownership), so owner of value ("hello") is some_str and not my_str, rust will use rules of ownership and will delete my_str from the stack memory

    // below code will give error
    // we can use my_str.clone() to avoid this error
    // println!("{}", my_str);

    // Create a Vec<i32>, push three values, then pop one and print remaining values.

    let mut v: Vec<i32> = Vec::new();

    // https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
    
    for i in (0..=50).step_by(10) {
        v.push(i);
    }

    println!("Before => {:?} and length is {}", v, v.len());
    v.pop();
    println!("After => {:?} and length is {}", v, v.len());

    // Use a for loop with a range to calculate the sum of numbers from 1 to 5.

    let mut sum_of_numbers: i32 = 0;
    for i in 1..=5 {
        sum_of_numbers = sum_of_numbers + i;
    }

    println!("Sum of 1 to 5 is {}", sum_of_numbers);

}

// Write a function that takes two integers, adds them, and returns the sum.

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

// Create a struct called Point with x and y fields, then instantiate and print it.

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

// Create an array of 5 integers, iterate over it, and print each value.

// method 1 -> if we want index values
fn iterate_array(arr: [i32; 5]) {
    for i in 0..arr.len() {
        println!("index : {} and value : {}", i, arr[i]);
    }
}

// method 2 -> if we dont want index values
fn iterate_array_2(arr: [i32; 5]) {
    for i in arr.iter() {
        println!("Value : {}", i);
    }
}

// Write code that demonstrates ownership transfer (move)

fn takes_ownership(some_str: String) {
    println!("{}", some_str);
} 

// Borrow a value by reference so that ownership is not transferred.

// we can use "&" we take the reference, this way it cant transfer the ownership

// passing "&" in function
fn calculate_length(s: &String) -> usize {

    // the below line is same as
    // return s.len();
    s.len()
}

