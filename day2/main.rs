// rust is good at memory management
// it asks you if you only need 1 - 10 numbers why do you need big memory for storing this.

// its like using 10 litres of bottle for storing 1 litre of water

// i8 -> --------, at each dash we can take either 0 or 1

// A signed integer means it can store both positive and negative values.

//     The first bit is reserved for the sign:

//         0 → positive

//         1 → negative

// i8 can store - -128 to 127

// 1 bit is for sign

// 7 bits are for value → 2⁷ = 128 values in each direction


// 2. Unsigned Integer

// An unsigned integer means it can store only positive values (including zero).

//     All bits are used for the value, so you get a higher positive range.

// u8 can store 0 - 255

// by default it takes i32, 
// if float then f8, f16...


// all variables are immutable by default
// primitive -> string, number, boolean

fn main() {
    let signed_number: i8 = -5;
    println!("Signed number : {}", signed_number);

    let unsigned_number: u8 = 5;
    println!("Unsigned number : {}", unsigned_number);

    let float_number: f32 = 100.1;
    println!("float number: {}", float_number);

    let hello: bool = true;
    if hello {
        println!("he said hello!!")
    } else {
        println!("he said bye")
    }

    let greeting: String = String::from("Hello, world!!");
    println!("Greetings : {}", greeting)
}

// https://dev.to/mrizwanashiq/primitive-and-non-primitive-56n8