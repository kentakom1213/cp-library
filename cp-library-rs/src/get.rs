//! Input macro for competitive programming.
//!
//! ## References
//!
//! - [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)

/// Reads values from standard input.
///
/// ## Supported forms
///
/// - `get!(T)`: reads one value of type `T`
/// - `get!(T1, T2, ...)`: reads multiple values from one line
/// - `get!(T; n)`: reads `n` lines, one value per line
/// - `get!(T1, T2; n)`: reads `n` tuples
/// - `get!(T ;;)`: reads one line into `Vec<T>`
/// - `get!(T ;; n)`: reads `n` lines into `Vec<Vec<T>>`
#[macro_export]
macro_rules! get {
    (parse, $val:expr, usize1) => {(get!(parse, $val, usize) - 1)};
    (parse, $val:expr, chars) => {get!(parse, $val, String).chars().collect::<Vec<_>>()};
    (parse, $val:expr, $t:ty) => {$val.parse::<$t>().unwrap()};
    ($p:tt) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            get!(parse, line.trim(), $p)
    }};
    ($($p:tt),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            let mut iter = line.split_whitespace();
            ( $(get!(parse, iter.next().unwrap(), $p),)* )
    }};
    ($t:tt ; $n:expr) => {(0..$n).map(|_| get!($t)).collect::<Vec<_>>()};
    ($($t:tt),* ; $n:expr) => {(0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()};
    ($t:tt ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            line.split_whitespace().map(|v| get!(parse, v, $t)).collect::<Vec<_>>()
    }};
    ($t:tt ;; $n:expr) => {(0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()};
}
