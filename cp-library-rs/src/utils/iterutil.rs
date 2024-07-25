//! イテレータのutil

use std::fmt::Display;

/// イテレータのツール
pub trait IterUtil: Iterator
where
    Self::Item: Display,
{
    /// 文字列として結合する
    fn join(&mut self, sep: &str) -> String {
        self.fold(String::new(), |mut s, v| {
            if s.is_empty() {
                s = format!("{}", v);
            } else {
                s += &format!("{}{}", sep, v);
            }
            s
        })
    }
}

impl<I: Iterator> IterUtil for I where I::Item: Display {}
