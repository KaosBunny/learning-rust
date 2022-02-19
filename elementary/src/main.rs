use std::io;

fn main() {
    question_six();
}

fn get_input(prompt: &str) -> String {
    let mut user_in = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut user_in)
        .expect("Unable to read lines.");

    user_in
}

fn question_six() {
    let choice = get_input("Would you like to calculate a summation or print a product of n? (sum/prod)");
    let n = get_input("Please pick a number: ");

    let n: i32 = n.trim().parse().expect("Not a number.");
    let mut out = 0;

    if choice.trim().to_lowercase() == "sum" {
        for i in 1..n + 1 {
            out += i;
        }
    } else if choice.trim().to_lowercase() == "prod" {
        out = 1;
        for i in 1..n + 1 {
            out *= i;
        }
    } else {
        println!("Sorry that wasn't a valid option.")
    }

    println!("Your output is: {}", out);
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

// fn question_four() {
//     println!("Pick a number!");
//     let mut user_in = String::new();

//     io::stdin()
//         .read_line(&mut user_in)
//         .expect("Unable to read line.");

//     let user_in: i32 = user_in.trim().parse().expect("Please type a number!");
//     let mut n = 0;

//     for i in 1..user_in + 1 {
//         n += i;
//     }

//     println!("Your final number is {}.", n);
// }

// fn question_five() {
//     println!("Pick a number!");
//     let mut user_in = String::new();

//     io::stdin()
//         .read_line(&mut user_in)
//         .expect("Unable to read lines.");

//     let user_in: i32 = user_in.trim().parse().expect("Please type in a number!");
//     let mut n = 0;

//     for i in 1..user_in + 1 {
//         if i % 5 == 0 || i % 3 == 0 {
//             n += i;
//         }
//     }

//     println!("Your final number is {}.", n);
// }

// fn question_six() {
//     println!("Would you like to calculate a summation or print product of n? (sum/prod)");

//     let mut choice = String::new();
//     io::stdin()
//         .read_line(&mut choice)
//         .expect("Unable to read line.");

//     println!("Please pick a number: ");
//     let mut n = String::new();
//     io::stdin()
//         .read_line(&mut n)
//         .expect("Unable to read line.");

//     let n: i32 = n.trim().parse().expect("Not a number.");
//     let mut out = 0;

//     if choice.trim().to_lowercase() == "sum" {
//         for i in 1..n + 1 {
//             out += i;
//         }
//     } else if choice.trim().to_lowercase() == "prod" {
//         out = 1;
//         for i in 1..n + 1 {
//             out *= i;
//         }
//     } else {
//         println!("Sorry that wasn't a valid option.")
//     }

//     println!("Your output is: {}", out);
// }
