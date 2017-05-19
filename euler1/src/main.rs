fn main() {

    let upper_bound = 1000;

    let sum: i32 = (1..upper_bound).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("Sum: {}", sum);
}
