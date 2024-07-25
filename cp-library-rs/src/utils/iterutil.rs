//! イテレータのutil

use std::fmt::{Debug, Display};

/// イテレータのツール
pub trait IterUtil: Iterator {
    /// 文字列として結合する
    fn join(&mut self, sep: &str) -> String
    where
        Self::Item: Display,
    {
        self.fold(String::new(), |mut s, v| {
            if s.is_empty() {
                s = format!("{}", v);
            } else {
                s += &format!("{}{}", sep, v);
            }
            s
        })
    }

    /// 文字列として結合する（デバッグ）
    fn join_debug(&mut self, sep: &str) -> String
    where
        Self::Item: Debug,
    {
        self.fold(String::new(), |mut s, v| {
            if s.is_empty() {
                s = format!("{:?}", v);
            } else {
                s += &format!("{}{:?}", sep, v);
            }
            s
        })
    }
}

impl<I: Iterator> IterUtil for I {}
