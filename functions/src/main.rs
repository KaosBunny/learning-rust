// fn pass_me_an_int(x: &mut u32) {
//     println!("Your value for x is {}.", x);

//     *x = *x * 2;
//     println!("Your value for x has changed to {}.", x)
// }
#![allow(unused_mut)]

fn main() {
    let mut s = String::from("Hello, World!");
    println!("Original string: {}", s);

    let r1 = &s;
    let r2 = &s;
    let mut r3 = &s;

    // Unchanged

    println!("This is r1: {}", r1);
    println!("This is r2: {}", r2);
    println!("This is r3: {}", r3);
}
