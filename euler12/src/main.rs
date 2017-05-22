mod triangle_numbers;

use triangle_numbers::Triangle;

fn divisors(num: usize) -> usize {
    let mut divisors = 2;

    let mut max_divisor = num;

    for d in 2..max_divisor {
        if d >= max_divisor {
            break
        }

        if num % d == 0 {
            max_divisor = num / d;
            divisors += if d != max_divisor { 2 } else { 1 };
        }
    }
    divisors
}

fn main() {

    let triangle = Triangle::new();

    for t in triangle {
        let d = divisors(t);

        if d > 500 {
            println!("{}", t);
            break;
        }
    }

}
