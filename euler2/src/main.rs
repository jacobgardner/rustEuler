mod fibonacci;

use fibonacci::Fibonacci;

fn main() {

    let mut sum = 0;

    for f in Fibonacci::new() {
        if f > 4000000 {
            break;
        }

        if f % 2 == 0 {
            sum += f;
        }
    }

    println!("Sum of fibs: {}", sum);

}
