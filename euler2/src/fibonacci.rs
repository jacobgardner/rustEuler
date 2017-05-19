
pub struct Fibonacci {
    prev: u64,
    current: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            prev: 0,
            current: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.prev + self.current;

        self.prev = self.current;
        self.current = next;

        Some(self.prev)
    }
}
