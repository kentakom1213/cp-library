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

/// ## ランレングス圧縮 (from Iterator)
/// - イテレータからエンコードを行う
pub fn run_length_encode_from<T, I>(mut itr: I) -> Vec<(T, usize)>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    let mut res = vec![];
    let mut cur = itr.next().unwrap();
    let mut cnt = 1;
    for val in itr {
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
