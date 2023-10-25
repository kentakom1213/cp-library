#![allow(dead_code)]

/// # AffineMatrix
/// アフィン変換（3x3行列）
#[derive(Debug, Clone, Copy)]
pub struct AffineMatrix {
    arr: [[isize; 3]; 3],
}

impl AffineMatrix {
    pub fn e() -> Self {
        Self {
            arr: [[1, 0, 0],
                  [0, 1, 0],
                  [0, 0, 1]]
        }
    }

    pub fn rotate90() -> Self {
        Self {
            arr: [[0, -1, 0],
                  [1, 0, 0],
                  [0, 0, 1]]
        }
    }

    pub fn rotate270() -> Self {
        Self {
            arr: [[0, 1, 0],
                  [-1, 0, 0],
                  [0, 0, 1]]
        } 
    }

    pub fn mirror_x(p: isize) -> Self {
        Self {
            arr: [[-1, 0, 2*p],
                  [0, 1, 0],
                  [0, 0, 1]]
        }  
    }

    pub fn mirror_y(p: isize) -> Self {
        Self {
            arr: [[1, 0, 0],
                  [0, -1, 2*p],
                  [0, 0, 1]]
        }  
    }

    pub fn dot(&self, other: &Self) -> Self {
        let mut arr = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    arr[i][j] += self.arr[i][k] * other.arr[k][j];
                }
            }
        }
        Self { arr }
    }

    pub fn apply(&self, vec: (isize, isize)) -> (isize, isize) {
        (
            self.arr[0][0] * vec.0 + self.arr[0][1] * vec.1 + self.arr[0][2],
            self.arr[1][0] * vec.0 + self.arr[1][1] * vec.1 + self.arr[1][2],
        )
    }
}
