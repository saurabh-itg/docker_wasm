fn fibonacci(n: u64) -> u64 {
    let mut fib = (0, 1);
    for _ in 0..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.0
}

fn main() {
    let n = 47;
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}

