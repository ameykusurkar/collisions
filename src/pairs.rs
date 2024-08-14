use std::ops::Range;

pub struct Pairs {
    outer: Range<usize>,
    inner: Range<usize>,
    curr_outer: usize,
}

impl Pairs {
    pub fn new(mut outer: Range<usize>) -> Self {
        let curr_outer = outer.next().unwrap_or(outer.start);
        let inner = (curr_outer + 1)..outer.end;
        Self {
            curr_outer,
            outer,
            inner,
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
                self.inner = (self.curr_outer + 1)..(self.outer.end);
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
        let mut pairs = Pairs::new(4..4);
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn one() {
        let mut pairs = Pairs::new(1..2);
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn two() {
        let mut pairs = Pairs::new(5..7);
        assert_eq!(pairs.next(), Some((5, 6)));
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn it_works() {
        let mut pairs = Pairs::new(0..4);
        assert_eq!(pairs.next(), Some((0, 1)));
        assert_eq!(pairs.next(), Some((0, 2)));
        assert_eq!(pairs.next(), Some((0, 3)));
        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((1, 3)));
        assert_eq!(pairs.next(), Some((2, 3)));
        assert_eq!(pairs.next(), None);
    }
}
