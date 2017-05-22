#![allow(dead_code)]
#![feature(test)]

extern crate test;

use std::env;
use test::Bencher;

fn number_count() -> usize {
    match env::args().nth(1) {
        Some(n) => n.parse().unwrap_or(10),
        None => 10,
    }
}

fn method1(divisor_count: usize) -> usize {
    let mut last_number = vec![(0, 0); divisor_count];
    let mut max_number = 1;

    'outie: loop {
        let mut is_done = true;
        'innie: for idx in 0..divisor_count {
            let (mut multiplier, mut value) = last_number[idx];

            while value < max_number {
                is_done = false;
                multiplier += 1;
                value = multiplier * (idx + 1);
            }

            max_number = value;
            last_number[idx] = (multiplier, value);

        }

        if is_done {
            return max_number;
        }
    }
}

fn method2(divisor_count: usize) -> usize {
    let mut idx = 0;
    'outie: loop {
        idx += 1;

        for divisor in 1..divisor_count + 1 {
            if idx % divisor != 0 {
                continue 'outie;
            }
        }

        return idx;
    }
}

#[bench]
fn bench_method1(b: &mut Bencher) {
    b.iter(|| method1(10));
}

#[bench]
fn bench_method2(b: &mut Bencher) {
    b.iter(|| method2(10));
}

fn main() {
    let divisor_count = number_count();

    println!("{}", method1(divisor_count));
    println!("{}", method2(divisor_count));
}
