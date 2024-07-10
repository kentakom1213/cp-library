use cp_library_rs::get;

fn main() {
    let (a, b) = get!(isize, isize);

    println!("{}", a + b);
}
