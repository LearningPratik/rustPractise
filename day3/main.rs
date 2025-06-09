// Conditional

fn main() {
    let num = 9;

    if num % 2 == 0 {
        println!("{} is even", num)
    } else {
        println!("{} is odd", num)
    }

//     for i in 0..10 {
//         print!("{} ", i)
//     }
// }

fn main() {
    let name: String = String::from("my name is pratik")
    let first_word: () = get_first_word(name)
    println!("{}", first_word)
}

fn get_first_word(sentence: String) -> String{
    
    let mut ans: String = String::from("")

    for char: char in sentence.chars() {
        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans
}