fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn factorial_iterative(n: i64) -> i64 {
    let mut result: i64 = 1;
    for i in 1..n {
        result *= i
    }
    result
}
