use std::io;

fn main() {
    question_four();
}

// fn question_one() {
//     println!("Hello, World!");
// }

// fn question_two() {
//     println!("What's your name?");
//     let mut name = String::new();
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line.");

//     println!("Hello, {}", name);
// }

// fn question_three() {
//     println!("What's your name?");
//     let mut name = String::new();
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line.");
//     let name = name.trim();

//     if name == "Alice" || name == "Bob" {
//         println!("Hello, {}.", name);
//         return;
//     }
//     println!("Sorry, you aren't recognized...");
// }

fn question_four() {
    println!("Pick a number!");
    let mut user_in = String::new();

    io::stdin()
        .read_line(&mut user_in)
        .expect("Unable to read line.");

    let user_in: i32 = user_in.trim().parse().expect("Please type a number!");
    let mut n = 0;

    for i in 1..user_in + 1 {
        n += i;
    }

    println!("Your final number is {}.", n);
}
