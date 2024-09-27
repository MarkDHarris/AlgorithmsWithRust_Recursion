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
        let stop_time3 = start_time3.elapsed();

        println!("time elapsed: {:.2?}", stop_time3);
        println!("given bottoms-up fibonacci({}) = {}\n", n, result3);

        let start_time4 = Instant::now();
        let result4 = recursive_fibonacci(n);
        let stop_time4 = start_time4.elapsed();

        println!("time elapsed: {:.2?}", stop_time4);
        println!("recursive fibonacci({}) = {}\n", n, result4);
    }
}

fn recursive_fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    return recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2);
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
    for _i in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fibonacci_on_the_fly_10() {
        let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];
        assert_eq!(55, fibonacci_on_the_fly(&mut fill_on_the_fly_values, 10));
    }

    #[test]
    fn check_precalculated_fibonacci_10() {
        let precalculated_values: Vec<i64> = prefill_vector();
        assert_eq!(55, precalculated_fibonacci(&precalculated_values, 10));
    }

    #[test]
    fn check_fibonacci_bottom_up_10() {
        assert_eq!(55, fibonacci_bottom_up(10));
    }

    #[test]
    fn check_recursive_fibonacci_10() {
        assert_eq!(55, recursive_fibonacci(10));
    }
}
