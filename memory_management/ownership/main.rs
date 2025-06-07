// Every variable we create will have a heap in heap memory
// or any data on heap memory will have a owner on the stack memory

// Ex : s1 = String:from("hello")
// s1 => on stack pointing to "hello" on heap

// If the s1 goes out of scope, or gets deleted then the value it was pointing to also gets deleted.

// question ? it is given that if variable is out of scope or deleted, the grabage collected will delete its value, what's different ?

// let s1 = String::from("Hello");
// let s2 = s1;

// As we know one value = only one owner, but in above case there are 2 owners so, what rust will do ?

// rust says, one value = one owner, so for above example,
// as s2 now points to "hello", s1 will get cancelled automatically and now "hello" only has one owner (s1)

// fn main() {
//     let s1: String = String::from("Hello");
//     println!("Before assigning to other owner: {}", s1);

//     let s2: String = s1;
//     println!("Before assigning to other owner: {}", s2);

//     // it wont compile here, because s2 has borrowed its value from s1
//     // println!("Before assigning to other owner: {}", s1);

//     // to compile this code, do this => comment out above code line
// }

// this code will not work because of ownership

// fn main() {
//     let my_str: String = String::from("Hello");
//     take_ownership(my_str);
//     println!("{}", my_str);
// }

// fn take_ownership(some_str: String) {
//     println!("{}", some_str);    
// }

// can value go back to previous variable ? Yes

fn main() {
    let mut my_str: String = String::from("hello");
    my_str = take_ownership(my_str);
    println!("{}", my_str);
}

fn take_ownership(some_str: String) -> String {
    println!("{}", some_str);
    return some_str;
}