use std::fmt;

pub fn _hello_world() {
    println!("Hello World!\nI'm a Rustacean!");
}

pub fn _print_formatting_1() {
    println!("My name is {0}. {1} {0}.", "Bond", "James");
}

pub fn _print_formatting_2() {
    struct Pair(i32);
    // let pair = Pair(1);

    println!("The struct now prints: {}!", Pair(3).0); // Only works with one value.
}

pub fn _print_formatting_3() {
    let n = 3.1415926535897932;
    println!("Pi is roughtly {:.3}", n);
}

pub fn print_formatting_4() {
    struct Pair(i32, f64);
    // let pair = Pair(1);

    impl fmt::Display for Pair {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "V1 = {} V2 = {}", self.0, self.1)
        }
    }

    println!("The struct now prints: {}!", Pair(3, 3.14));
}
