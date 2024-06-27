// Rust provides type safety via static typing.
// Variables types are either given explicitly or inferred by the compiler.
fn main() {
    // Variable bindings are made with the keyword let.
    let x: i32 = 1;
    println!("x: {x}");

    // Variable are immutable by default, the keyword mut makes variables mutable.
    let mut y = 2;
    println!("y: {y}");
    y = 3;
    println!("y: {y}");
}
