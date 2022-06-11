use std::vec::Vec;

struct Factorization {
    divided: u64,
    max_factor: u64,
}

impl Factorization {
    fn new(divided: u64) -> Self {
        Self {
            divided: divided,
            max_factor: 2,
        }
    }
}

impl Iterator for Factorization {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.divided == 1 {
                return None;
            }

            if self.divided % self.max_factor == 0 {
                self.divided = self.divided / self.max_factor;
                return Some(self.max_factor);
            }

            self.max_factor += 1;
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let factor = Factorization::new(n);
    factor.collect()
}
