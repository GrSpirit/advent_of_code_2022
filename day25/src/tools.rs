use num_traits::{PrimInt, one};
pub(crate) struct DivIterator<T>{
    x: T,
    d: T,
    rem: u8,
}

impl<T> DivIterator<T> {
    pub(crate) fn new(x: T, d: T) -> Self {
        Self { x, d, rem: 0 }
    }
}

impl<T> Iterator for DivIterator<T> 
where T: PrimInt
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x.is_zero() {
            return None;
        }
        self.rem = self.x.rem(self.d).to_u8().unwrap();
        self.x = self.x.div(self.d);
        if self.rem >= 3 {
            self.x = self.x.add(one());
        }
        Some(self.rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_i32() {
        let mut iter = DivIterator::new(7, 5);
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn test_u64() {
        let mut iter = DivIterator::new(8u64, 5u64);
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
    }
}