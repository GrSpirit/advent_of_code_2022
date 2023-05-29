pub(crate) trait WrappingInc {
    fn inc(&mut self, n: usize);
}

pub(crate) trait WrappingDec {
    fn dec(&mut self, n: usize);
}

impl WrappingInc for usize {
    #[inline]
    fn inc(&mut self, n: usize) {
        *self = if *self + 1 >= n { 0 } else { *self + 1 };
    }
}

impl WrappingDec for usize {
    #[inline]
    fn dec(&mut self, n: usize) {
        *self = if *self == 0 { n - 1} else { *self - 1};
    }
}

pub(crate) struct Wrapper(pub usize);

impl WrappingInc for Wrapper {
    #[inline]
    fn inc(&mut self, n: usize) {
        self.0.inc(n);
    }
}

impl WrappingDec for Wrapper {
    #[inline]
    fn dec(&mut self, n: usize) {
        self.0.dec(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc() {
        let mut wi = Wrapper(1);
        wi.inc(3);
        assert_eq!(2, wi.0);
        wi.inc(3);
        assert_eq!(0, wi.0);
    }

    #[test]
    fn test_dec() {
        let mut wi = Wrapper(1);
        wi.dec(3);
        assert_eq!(0, wi.0);
        wi.dec(3);
        assert_eq!(2, wi.0);
    }
}