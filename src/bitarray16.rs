//! 64bit整数を16bit整数4つの配列として使う

/// 64bit整数を16bit整数4つの配列として使う
pub trait BitArray16 {
    fn set(&self, i: usize, x: u16) -> Self;
    fn get(&self, i: usize) -> u16;
}

impl BitArray16 for usize {
    fn set(&self, i: usize, x: u16) -> Self {
        *self & !(0xffff << (16 * i)) | (x as usize) << (16 * i)
    }
    fn get(&self, i: usize) -> u16 {
        ((*self & (0xffff << (16 * i))) >> (16 * i)) as u16
    }
}
