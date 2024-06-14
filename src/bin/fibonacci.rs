use recursion::get_i64;
use std::time::Instant;

fn main() {
    println!("Enter -1 to exit\n");

    loop {
        let n = get_i64("N: ");

        if n < 0 {
            break;
        }

        let start_time = Instant::now();
        let result = fibonacci(n);
        let stop_time = start_time.elapsed();

        println!("time elapsed: {:.2?}", stop_time);
        println!("fibonacci({}) = {}\n", n, result);
    }
}

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
