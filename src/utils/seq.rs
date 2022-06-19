pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { a: 1, b: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(c)
    }
}
