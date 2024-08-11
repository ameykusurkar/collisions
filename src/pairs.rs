pub struct Pairs {
    outer: std::ops::Range<usize>,
    inner: std::ops::Range<usize>,
    curr_outer: usize,
    limit: usize,
}

impl Pairs {
    pub fn new(limit: usize) -> Self {
        let mut outer = 0..limit;
        let curr_outer = outer.next().unwrap_or(0);
        let inner = (curr_outer + 1)..limit;
        Self {
            curr_outer,
            outer,
            inner,
            limit,
        }
    }
}

impl Iterator for Pairs {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let curr_inner = match self.inner.next() {
            Some(r) => r,
            None => {
                self.curr_outer = self.outer.next()?;
                self.inner = (self.curr_outer + 1)..self.limit;
                self.inner.next()?
            }
        };
        Some((self.curr_outer, curr_inner))
    }
}

#[cfg(test)]
mod tests {
    use super::Pairs;

    #[test]
    fn empty() {
        let mut pairs = Pairs::new(0);
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn one() {
        let mut pairs = Pairs::new(1);
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn two() {
        let mut pairs = Pairs::new(2);
        assert_eq!(pairs.next(), Some((0, 1)));
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn it_works() {
        let mut pairs = Pairs::new(4);
        assert_eq!(pairs.next(), Some((0, 1)));
        assert_eq!(pairs.next(), Some((0, 2)));
        assert_eq!(pairs.next(), Some((0, 3)));
        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((1, 3)));
        assert_eq!(pairs.next(), Some((2, 3)));
        assert_eq!(pairs.next(), None);
    }
}
