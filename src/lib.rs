#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<Q7> for f32 {
    /// Returns a float 32 value from a Q7 value
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32
    }
}

impl From<Q7> for f64 {
    /// Returns a float 64 value from a Q7 value
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    /// Returns a Q7 value from a float 32 value
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<f64> for Q7 {
    /// Returns a Q7 value from a float 64 value
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

// Unit test code
#[cfg(test)]
mod tests {
    use super::*;

    /// Test out of bound values
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    /// Test conversion of float 32 to Q7 values
    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    /// Test conversion of Q7 values to f32 values
    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
