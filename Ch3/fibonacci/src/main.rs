fn main() {
    let n = 6;
    println!("fib {} is {}!", n, fib(n));
}

fn fib(x: i32) -> i32 {
    if x == 1 || x == 2 {
        1
    }
    else {
        fib(x-1) + fib(x-2)
    }
}
