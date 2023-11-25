//! chminの実装

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_chmax() {
        // 2変数
        let mut a = 20;
        let b = 10;
        chmin!(a, b);
        assert_eq!(a, 10);

        // 5変数
        let mut one = 100;
        let two = 30;
        let three = 400;
        let four = 10;
        let five = 5;
        let has_changed = chmin!(one, two, three, four, five,);
        assert_eq!(has_changed, true);
        assert_eq!(one, 5);
    }

    #[test]
    fn test_chmax_vector() {
        let mut arr = (0..10).collect::<Vec<usize>>();
        println!("{:?}", &arr);

        for i in 0..10 {
            chmin!(arr[0], arr[i]);
        }

        println!("{:?}", &arr);
        assert_eq!(&arr, &vec![0_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
