use std::ops;

#[inline]
pub fn linear<I, O>(f1: I, f2: I, mut a: f32) -> O
where
    I: ops::Mul<f32, Output = O>,
    O: ops::Add<Output = O>,
{
    debug_assert!((0.0..=1.0).contains(&a), "a should be between 0 and 1");
    a = a.clamp(0.0, 1.0);

    f1 * (1.0 - a) + f2 * a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "a should be between 0 and 1")]
    fn test_a_gt_1() {
        let _ = linear(1.0, 1.0, 2.0);
    }

    #[test]
    #[should_panic(expected = "a should be between 0 and 1")]
    fn test_a_lt_0() {
        let _ = linear(1.0, 1.0, -1.2);
    }

    #[test]
    fn test_valid_lerp_a_eq_0() {
        let start = 1.4;
        let end = 6.2;
        let res = linear(start, end, 0.0);
        assert_eq!(start, res);
    }

    #[test]
    fn test_valid_lerp_a_eq_1() {
        let start = 1.4;
        let end = 6.2;
        let res = linear(start, end, 1.0);
        assert_eq!(end, res);
    }

    #[test]
    fn test_valid_lerp_a_eq_0_5() {
        let start = 2.0;
        let end = 3.0;
        let res = linear(start, end, 0.5);
        assert_eq!(2.5, res);
    }

    #[test]
    fn test_valid_lerp_a_eq_0_25() {
        let start = 2.0;
        let end = 3.0;
        let res = linear(start, end, 0.25);
        assert_eq!(2.25, res);
    }

    #[test]
    fn test_valid_lerp_a_eq_0_75() {
        let start = 2.0;
        let end = 3.0;
        let res = linear(start, end, 0.75);
        assert_eq!(2.75, res);
    }
}
