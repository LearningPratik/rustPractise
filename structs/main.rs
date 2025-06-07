// Structs => structure data together, similar to objects in JS

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("pratik"),
//         email: String::from("pratik@gmail.com"),
//         sign_in_count: 1
//     };

//     println!("User 1 username: {:?}", user1.username);
// }

// go through -> tuple struct, unit struct


// implementing struct

struct rectangle {
    width: u32,
    height: u32,
}


impl rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect = rectangle {
        width: 30,
        height: 20,
    };

    print!("The area of the rectangle is : {}", rect.area());
}