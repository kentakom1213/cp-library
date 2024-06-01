use cp_library_rs::sparse_table::*;
use rand::random;

#[test]
fn test_range_min() {
    let N = 9;
    let arr = vec![5, 7, 8, 0, 3, 4, 2, 2, 9];
    let table = SparseTable::<Alg::Min>::build(&arr);

    eprintln!("{:?}", table);

    let get_range = |l, r| *arr[l..r].iter().min().unwrap_or(&isize::MAX);

    for l in 0..N {
        for r in l..N {
            assert_eq!(table.get_range(l..r), get_range(l, r));
        }
    }
}

#[test]
fn test_range_max_random() {
    let N = 500;
    let arr = (0..N).map(|_| random()).collect::<Vec<isize>>();
    let table = SparseTable::<Alg::Max>::build(&arr);

    let get_range = |l, r| *arr[l..r].iter().max().unwrap_or(&isize::MAX);

    for l in 0..N {
        for r in l..N {
            assert_eq!(table.get_range(l..r), get_range(l, r));
        }
    }
}
