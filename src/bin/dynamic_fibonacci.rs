use recursion::get_i64;
use std::time::Instant;

fn main() {
    println!("Enter -1 to exit\n");

    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];
    let precalculated_values: Vec<i64> = prefill_vector();

    loop {
        let n = get_i64("N: ");

        if n < 0 {
            break;
        }

        let start_time1 = Instant::now();
        let result1 = fibonacci_on_the_fly(&mut fill_on_the_fly_values, n);
        let stop_time1 = start_time1.elapsed();

        println!("time elapsed: {:.2?}", stop_time1);
        println!("fibonacci on the fly({}) = {}\n", n, result1);

        let start_time2 = Instant::now();
        let result2 = precalculated_fibonacci(&precalculated_values, n);
        let stop_time2 = start_time2.elapsed();

        println!("time elapsed: {:.2?}", stop_time2);
        println!("pre-calculated fibonacci({}) = {}\n", n, result2);

        let start_time3 = Instant::now();
        let result3 = fibonacci_bottom_up(n);
        let stop_time3 = start_time2.elapsed();

        println!("time elapsed: {:.2?}", stop_time3);
        println!("given bottoms-up fibonacci({}) = {}\n", n, result3);
    }
}

fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    // if we don't have the value already calculated, calculate it, store it, then use it
    // if we do have it calculated already, just use it
    if (values.len() as i64) <= n {
        let new_n = fibonacci_on_the_fly(values, n - 1) + fibonacci_on_the_fly(values, n - 2);
        values.push(new_n);
    }

    values[n as usize]
}

fn precalculated_fibonacci(precalculated_values: &Vec<i64>, n: i64) -> i64 {
    precalculated_values[n as usize]
}

fn prefill_vector() -> Vec<i64> {
    let mut values = vec![0, 1];

    for i in 2..92 {
        let values_at_i = values[i - 1] + values[i - 2];
        values.push(values_at_i);
    }

    values
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for i in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}
