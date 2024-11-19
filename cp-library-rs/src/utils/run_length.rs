//! ランレングス圧縮

/// ## ランレングス圧縮
/// - スライスからエンコードを行う
pub fn run_length_encode<T>(arr: &[T]) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut res = vec![];
    let mut cur = arr[0];
    let mut cnt = 1;
    for &val in &arr[1..] {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cur, cnt));
            cur = val;
            cnt = 1;
        }
    }
    let last_elem = *arr.last().unwrap();
    res.push((last_elem, cnt));

    res
}

/// ## RunLength
/// - イテレータに対してランレングス圧縮を実装する
pub trait RunLength: Iterator {
    /// ## ランレングス圧縮 (from Iterator)
    /// - イテレータからエンコードを行う
    fn run_length_encode(&mut self) -> Vec<(Self::Item, usize)>
    where
        Self::Item: PartialEq,
    {
        let mut res = vec![];
        let mut cur = self.next().unwrap();
        let mut cnt = 1;
        for val in self {
            if val == cur {
                cnt += 1;
            } else {
                res.push((cur, cnt));
                cur = val;
                cnt = 1;
            }
        }
        res.push((cur, cnt));

        res
    }
}

impl<I: Iterator> RunLength for I {}
