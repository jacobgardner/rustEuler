
pub struct Triangle {
    value: usize,
    index: usize,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            value: 0,
            index: 0,
        }
    }
}

impl Iterator for Triangle {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {

        self.index += 1;
        self.value += self.index;

        Some(self.value)
    }
}

