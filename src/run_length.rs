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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_usize() {
        let arr = vec![0, 1, 1, 3, 3, 3, 2, 2, 1, 5, 9, 0];
        let comp = run_length_encode(&arr);
        let ans = vec![
            (0, 1),
            (1, 2),
            (3, 3),
            (2, 2),
            (1, 1),
            (5, 1),
            (9, 1),
            (0, 1),
        ];

        assert_eq!(comp, ans);
    }

    #[test]
    fn test_string() {
        let strs = vec![
            "Welcome", "to", "Moo", "Moo", "Moo", "nsi", "nsi", "nsi", "nsi", "...", "nside.",
        ];
        let comp = run_length_encode(&strs);
        let ans = vec![
            ("Welcome", 1),
            ("to", 1),
            ("Moo", 3),
            ("nsi", 4),
            ("...", 1),
            ("nside.", 1),
        ];
        // [引用] "Mother2", nintendo, 1989

        assert_eq!(comp, ans);
    }

    #[test]
    fn test_chars() {
        let str = "aaaxbbbbbbccddef";
        let chars: Vec<char> = str.chars().collect();
        let comp = run_length_encode(&chars);
        let ans = vec![
            ('a', 3),
            ('x', 1),
            ('b', 6),
            ('c', 2),
            ('d', 2),
            ('e', 1),
            ('f', 1),
        ];

        assert_eq!(comp, ans);
    }

    #[test]
    fn test_chars_from_iter() {
        let str = "aaaxbbbbbbccddef";
        let chars: Vec<char> = str.chars().collect();
        let comp = run_length_encode_from(chars.iter());
        let ans = vec![
            (&'a', 3),
            (&'x', 1),
            (&'b', 6),
            (&'c', 2),
            (&'d', 2),
            (&'e', 1),
            (&'f', 1),
        ];

        println!("{:?}", comp);
        assert_eq!(comp, ans);
    }
}
