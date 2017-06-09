pub struct Tripletizer {
    n: i32,
}

impl Tripletizer {
    pub fn new(n: i32) -> Tripletizer {
        Tripletizer { n: n }
    }
}

impl Iterator for Tripletizer {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let result = self.n % 1000;
            self.n /= 1000;
            Some(result)
        } else {
            None
        }
    }
}
