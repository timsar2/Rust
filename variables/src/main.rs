fn main() {
    const X: i32 = 6;
    let fibonachi = fibo_rust(X);
    println!("fib {} is {}", X, fibonachi);
}

fn fibo_rust(x: i32) -> i32 {
    let mut fib: [i32; 3] = [0, -1, 1];
    for i in 0..x {
        fib[0] = fib[1];
        fib[1] = fib[2];
        fib[2] = fib[0] + fib[1];
        println!("{}: {} ",i ,fib[2]);
    }
    fib[2]
}
