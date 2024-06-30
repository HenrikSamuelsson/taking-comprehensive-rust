// Will panic when the addition overflows for large Fibonacci numbers that do not fit in a u32.
// Fibonacci number 47 works, but not 48 that is 4,807,526,976 which is larger than the maximum u32 
// being 4,294,967,295.
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        n
    } else {
        // The recursive case.
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 48;
    println!("fib({n}) = {}", fib(n));
}
