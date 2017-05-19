mod fibonacci;

use fibonacci::Fibonacci;

fn main() {
    let sum = Fibonacci::new()
        .take_while(|&elem| elem < 4000000)
        .fold(0, |sum, f| if f % 2 == 0 { sum + f } else { sum });

    println!("Sum of fibs: {}", sum);
}
