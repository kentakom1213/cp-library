//! usizeの便利ツール

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abs_diff() {
        assert_eq!(5.abs_diff(4), 1);
        assert_eq!(1.abs_diff(9), 8);
        assert_eq!(9238712.abs_diff(98370918237), 98361679525);
        assert_eq!(1092387.abs_diff(9238728), 8146341);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(0.sqrt(), 0);
        assert_eq!(4.sqrt(), 2);
        assert_eq!(5.sqrt(), 2);
        assert_eq!(100.sqrt(), 10);
        assert_eq!(101.sqrt(), 10);
        assert_eq!(12390879108273.sqrt(), 3520068);
        assert_eq!(99121929823792.sqrt(), 9955999);
        assert_eq!(1001001001001001001.sqrt(), 1000500375);
    }
}
