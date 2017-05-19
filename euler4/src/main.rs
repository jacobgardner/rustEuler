fn is_palindrome(input: u64) -> bool {
    let stringied = input.to_string();

    let a: String = stringied.chars().rev().collect();

    stringied == a
}

fn main() {

    let mut palindrome = 0;

    for a in 100..1000 {
        for b in 100..1000 {
            let mult = a * b;

            if mult > palindrome && is_palindrome(mult) {
                palindrome = mult;
            }
        }
    }

    println!("{}", palindrome);
}
