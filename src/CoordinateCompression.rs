#![allow(dead_code)]

/// # 座標圧縮
#[derive(Debug)]
pub struct Compression<'a, T> {
    pub size: usize,
    pub sorted_array: Vec<&'a T>,
}

impl<'a, T: Ord> Compression<'a, T> {
    pub fn new(array: &'a [T]) -> Self {
        let mut comp: Vec<&T> = array.iter().collect();
        comp.sort();
        comp.dedup();
        Self {
            size: comp.len(),
            sorted_array: comp,
        }
    }

    /// 圧縮後の番号を返す
    pub fn idx(&self, val: &T) -> Option<usize> {
        let idx = self.sorted_array.binary_search(&val);
        if let Ok(idx) = idx {
            Some(idx)
        } else {
            None
        }
    }

    /// 圧縮前の要素を返す
    pub fn val(&self, idx: usize) -> Option<&T> {
        if idx < self.size {
            Some(self.sorted_array[idx])
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {
    use super::Compression;

    #[test]
    fn test_compression_i32() {
        let arr = vec![6, 7, 100, 10, 4, 100, 20, 0, 300, 0];
        let comp = Compression::new(&arr);

        println!("{:?}", &comp);

        // idxのテスト
        assert_eq!(comp.idx(&0), Some(0));
        assert_eq!(comp.idx(&5), None);
        assert_eq!(comp.idx(&20), Some(5));
        assert_eq!(comp.idx(&300), Some(7));
        assert_eq!(comp.idx(&400), None);

        // valのテスト
        assert_eq!(comp.val(0), Some(&0));
        assert_eq!(comp.val(5), Some(&20));
        assert_eq!(comp.val(10), None);
    }

    #[test]
    fn test_compression_val() {
        let arr = vec!["a", "zoo", "hello", "nagoya", "newyork", "a", "zoo"];
        let comp = Compression::new(&arr);

        println!("{:?}", &comp);

        // idxのテスト
        assert_eq!(comp.idx(&"a"), Some(0));
        assert_eq!(comp.idx(&"zoo"), Some(4));
        assert_eq!(comp.idx(&"akita"), None);

        // valのテスト
        assert_eq!(comp.val(2), Some(&"nagoya"));
        assert_eq!(comp.val(3), Some(&"newyork"));
        assert_eq!(comp.val(10), None);
    }
}
