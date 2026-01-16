#![allow(non_snake_case)]

use cp_library_rs::{
    convolution::ntt::FFT,
    number_theory::modint::{Modint, M998},
};
use rand::{rng, Rng};
use rstest::rstest;

#[test]
fn test_fft() {
    {
        let arr: Vec<M998> = vec![Modint(1), Modint(2), Modint(3), Modint(4)];

        let res = FFT::fft(&arr).unwrap();
        eprintln!("fft({:?}) = {:?}", arr, res);

        let res2 = FFT::ifft(&res).unwrap();
        eprintln!("ifft({:?}) = {:?}", res, res2);

        assert_eq!(res2, arr);
    }

    {
        let arr: Vec<M998> = vec![
            Modint(3),
            Modint(1),
            Modint(4),
            Modint(1),
            Modint(5),
            Modint(9),
        ];

        let res = FFT::fft(&arr).unwrap();
        eprintln!("fft({:?}) = {:?}", arr, res);

        let res2 = FFT::ifft(&res).unwrap();
        eprintln!("ifft({:?}) = {:?}", res, res2);

        let arr_ext = vec![3, 1, 4, 1, 5, 9, 0, 0];
        assert_eq!(res2, arr_ext);
    }

    {
        let arr: Vec<M998> = vec![
            Modint(31415),
            Modint(92653),
            Modint(58979),
            Modint(32384),
            Modint(62643),
            Modint(38327),
            Modint(95028),
        ];

        let res = FFT::fft(&arr).unwrap();
        eprintln!("fft({:?}) = {:?}", arr, res);

        let res2 = FFT::ifft(&res).unwrap();
        eprintln!("ifft({:?}) = {:?}", res, res2);

        let arr_ext = vec![31415, 92653, 58979, 32384, 62643, 38327, 95028, 0];
        assert_eq!(res2, arr_ext);
    }

    {
        let arr = vec![
            Modint(31415926),
            Modint(53589793),
            Modint(23846264),
            Modint(33832795),
            Modint(2884197),
            Modint(16939937),
            Modint(51058209),
            Modint(74944592),
        ];

        let res = FFT::fft(&arr).unwrap();
        eprintln!("fft({:?}) = {:?}", arr, res);

        let res2 = FFT::ifft(&res).unwrap();
        eprintln!("ifft({:?}) = {:?}", res, res2);

        assert_eq!(res2, arr);
    }
}

#[rstest(
    size,
    p,
    case(500, 5767169),
    case(500, 5767169),
    case(3000, 5767169),
    case(500, 998244353),
    case(500, 998244353),
    case(3000, 998244353),
    // too large case
    case(200000, 998244353),
    case(200000, 998244353),
    case(200000, 998244353),
)]
fn test_fft_large(size: usize, p: usize) {
    let mut rng = rng();

    let arr: Vec<M998> = (0..size)
        .map(|_| rng.random_range(0..p))
        .map(Modint::from)
        .collect();

    let res = FFT::fft(&arr).unwrap();
    let res2 = FFT::ifft(&res).unwrap();

    assert_eq!(&res2[..size], arr);
}
