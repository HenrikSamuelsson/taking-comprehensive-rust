fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    // Here x and y are not given any type when declared.
    let x = 10;
    let y = 20;

    // Rust will infer the type from its usage here.
    takes_u32(x);
    takes_i8(y);
}
