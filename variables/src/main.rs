fn main() {
    let x = 3; // This is the default immutable variable.
    let mut y = 1; // This is the mutable variable with the mut keyword.

    y = y + x;

    println!("The value of the mutable variable is {} and the value of the immutable variable is {}.", y, x);

    // We can also make constants which are variables that, like standard variables, are immutable,
    // but unlike standard variables, must be known at compile time.
    // They must also have an explicitly stated type. This can be done with this syntax.
    // const foo: type = value;
    // let bar: type = value;
    // let mut foo: type = value;
    const SPEED_OF_LIGHT: u32 = 299792458;

    //You can also shadow variables by "reassigning" a standard variable in a different scope. The variable
    // will then revert to it's original value when going out ot scope.
    let foo = SPEED_OF_LIGHT;
    if foo < 300000000 {
        let foo = "3";
        println!("Foo = {}", foo);
    }
    println!("Foo = {}", foo);
}
