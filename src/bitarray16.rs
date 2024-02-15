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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut arr = 0;

        println!("{:0>64b}", arr);
        assert_eq!(arr, 0);
        assert_eq!(arr.get(0), 0);
        assert_eq!(arr.get(1), 0);
        assert_eq!(arr.get(2), 0);
        assert_eq!(arr.get(3), 0);

        arr = arr.set(0, 1024);
        println!("{:0>64b}", arr);
        assert_eq!(arr, 0b_10000000000);
        assert_eq!(arr.get(0), 1024);
        assert_eq!(arr.get(1), 0);
        assert_eq!(arr.get(2), 0);
        assert_eq!(arr.get(3), 0);

        arr = arr.set(1, 1023);
        println!("{:0>64b}", arr);
        assert_eq!(arr, 0b_0000001111111111_0000010000000000);
        assert_eq!(arr.get(0), 1024);
        assert_eq!(arr.get(1), 1023);
        assert_eq!(arr.get(2), 0);
        assert_eq!(arr.get(3), 0);

        arr = arr.set(3, 0xffff);
        println!("{:0>64b}", arr);
        assert_eq!(
            arr,
            0b_1111111111111111_0000000000000000_0000001111111111_0000010000000000
        );
        assert_eq!(arr.get(0), 1024);
        assert_eq!(arr.get(1), 1023);
        assert_eq!(arr.get(2), 0);
        assert_eq!(arr.get(3), 65535);
    }
}
