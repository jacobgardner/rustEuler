fn main() {


    let square_sum: u64 = (1..100).sum::<u64>().pow(2);
    let sum_square: u64 = (1..100).map(|i| i * i).sum();

    println!("{}", square_sum - sum_square);

}
