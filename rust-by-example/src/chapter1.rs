#[allow(dead_code)]

pub fn hello_world() {
    println!("Hello World!\nI'm a Rustacean!");
}

pub fn print_formatting_1() {
    println!("My name is {0}. {1} {0}.", "Bond", "James");
}

pub fn print_formatting_2() {
    struct Pair(i32);
    // let pair = Pair(1);

    println!("The struct now prints: {}!", Pair(3).0);
}
