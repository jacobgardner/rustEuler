fn prime_factors(mut num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();


    for divisor in 2..num {
        if num == divisor {
            break;
        } else if num % divisor == 0 {
            factors.push(divisor);

            num /= divisor;
        }
    }

    factors.push(num);

    factors
}

fn main() {
    println!("{}", prime_factors(600851475143).iter().max().unwrap());
}
