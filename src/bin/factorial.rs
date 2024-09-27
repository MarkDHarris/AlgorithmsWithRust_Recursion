fn main() {
    for n in 0..=20 {
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

#[allow(dead_code)]
fn factorial_iterative(n: i64) -> i64 {
    let mut result: i64 = 1;
    for i in 1..=n {
        result *= i
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_recursive_factorial_20() {
        assert_eq!(1_307_674_368_000, factorial(15));
        assert_eq!(2432902008176640000, factorial(20));
    }

    #[test]
    fn check_iterative_factorial_20() {
        assert_eq!(1_307_674_368_000, factorial_iterative(15));
        assert_eq!(2432902008176640000, factorial_iterative(20));
    }
}
