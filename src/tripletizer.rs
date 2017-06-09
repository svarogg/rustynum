struct Tripletizer {
    n: i32,
}

impl Iterator for Tripletizer {
    fn next(&mut self) -> Option<i32> {
        if self.n > 0 {
            let result = self.n % 1000;
            self.n /= 1000;
            Some(result)
        } else {
            None
        }
    }
}
