use cp_library_rs::multiset_splay_tree::MultiSet;
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        T: String
    }

    let mut set = T
        .chars()
        .enumerate()
        .fold(MultiSet::new(), |mut set, (x, e)| {
            if e == '1' {
                set.insert(x as isize);
            }
            set
        });

    for _ in 0..Q {
        input! {
            c: usize,
            k: isize
        }

        match c {
            0 => {
                set.insert(k);
            }
            1 => {
                set.delete(&k);
            }
            2 => {
                println!("{}", set.get(&k).is_some() as usize);
            }
            3 => {
                let res = *set.lower_bound(&k).unwrap_or(&(-1));
                println!("{res}");
            }
            4 => {
                let res = *set.lower_bound_rev(&k).unwrap_or(&(-1));
                println!("{res}");
            }
            _ => (),
        }
    }
}
