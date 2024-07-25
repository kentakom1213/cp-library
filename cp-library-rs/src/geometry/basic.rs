//! 点、直線等に関する基礎的な操作

use super::vec2::Vec2;

/// 十分に小さい数
pub const EPS: f64 = 1e-8;

/// 点（2次元平面）
pub type Point = Vec2<f64>;

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.dist(other.clone()) < EPS
    }
}

impl Point {
    /// 2点間のユークリッド距離を求める
    ///
    /// ```math
    /// \|\boldsymbol{a} - \boldsymbol{b}\| = \sqrt{(a_x - b_x)^2 + (a_y - b_y)^2}
    /// ```
    pub fn dist(&self, other: Self) -> f64 {
        self.dist2(other).sqrt()
    }

    /// ノルムを求める
    ///
    /// ```math
    /// \|\boldsymbol{a}\| = \sqrt{a_x^2 + a_y^2}
    /// ```
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    /// 同じ向きの単位ベクトルを求める
    pub fn unit(&self) -> Self {
        self.clone() / self.norm()
    }

    /// 法線ベクトル
    /// （90度回転させたベクトル）
    /// を求める
    pub fn normal(&self) -> Self {
        let Self(x, y) = *self;
        Self(-y, x)
    }

    /// 原点周りに $`\theta`$ 回転させた点を求める
    pub fn rotate_o(&self, theta: f64) -> Self {
        let Self(x, y) = *self;
        Self(
            theta.cos() * x - theta.sin() * y,
            theta.sin() * x + theta.cos() * y,
        )
    }
}

/// 直線（2次元平面）
#[derive(Debug)]
pub struct Line(pub Point, pub Point);

impl Line {
    /// 射影
    /// （直線`l`に対して点`p`から引いた垂線の足）
    /// を求める
    pub fn projection(&self, p: Point) -> Point {
        let Self(a, b) = *self;
        let t = (p - a).dot(a - b) / (a - b).norm2();
        a + (a - b) * t
    }

    /// 反射
    /// （直線`l`に対して点`p`に対称な点）
    /// を求める
    pub fn reflection(&self, p: Point) -> Point {
        p + (self.projection(p) - p) * 2.0
    }

    /// 2直線の直交判定
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        (self.1 - self.0).dot(other.1 - other.0).abs() < EPS
    }

    /// 2直線の平行判定
    pub fn is_parallel(&self, other: &Self) -> bool {
        (self.1 - self.0).cross(other.1 - other.0).abs() < EPS
    }

    /// 2直線の交点を求める
    ///
    /// **戻り値**
    /// - `Point` : 交点
    pub fn cross_point(&self, other: &Self) -> Point {
        let d1 = (self.1 - self.0).cross(other.1 - other.0);
        let d2 = (self.1 - self.0).cross(self.1 - other.0);

        if d1.abs() < EPS && d2.abs() < EPS {
            return other.0;
        }

        other.0 + (other.1 - other.0) * (d2 / d1)
    }

    /// 点`p`と直線の距離を求める
    pub fn dist_point(&self, p: Point) -> f64 {
        let Self(a, b) = *self;
        (b - a).cross(p - a).abs() / a.dist(b)
    }
}

/// 3点 O, A, B の位置関係
#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    /// O, A, B が反時計回りになる場合
    ///
    /// > ```text
    /// >  B   A
    /// >  ↑ ➚
    /// >  O
    /// > ```
    CounterClockwise,
    /// O, A, B が時計回りになる場合
    ///
    /// > ```text
    /// >  A   B
    /// >  ↑ ➚
    /// >  O
    /// > ```
    Clockwise,
    /// B, O, A がこの順で同一直線上にある場合
    ///
    /// > ```text
    /// > B ← O → A
    /// > ```
    OnlineBack,
    /// O, A, B がこの順で同一直線上にある場合
    ///
    /// > ```text
    /// > O → A → B
    /// > ```
    OnlineFront,
    /// B が線分 OA 上にある場合
    ///
    /// > ```text
    /// > O → B → A
    /// > ```
    OnSegment,
}

impl Orientation {
    /// 3点 O, A, B の位置関係を調べる
    pub fn calc(O: Point, mut A: Point, mut B: Point) -> Self {
        A = A - O;
        B = B - O;

        if A.cross(B) > EPS {
            return Orientation::CounterClockwise;
        }

        if A.cross(B) < -EPS {
            return Orientation::Clockwise;
        }

        if A.dot(B) < 0.0 {
            return Orientation::OnlineBack;
        }

        if A.norm2() < B.norm2() {
            return Orientation::OnlineFront;
        }

        Orientation::OnSegment
    }

    /// 値を取得する
    pub fn val(&self) -> f64 {
        match self {
            Orientation::CounterClockwise => 1.0,
            Orientation::Clockwise => -1.0,
            Orientation::OnlineBack => 2.0,
            Orientation::OnlineFront => -2.0,
            Orientation::OnSegment => 0.0,
        }
    }
}

/// 線分（2次元平面）
#[derive(Debug, Clone, Copy)]
pub struct Segment(pub Point, pub Point);

impl Segment {
    /// 2つの線分が衝突しているかどうかを判定する
    pub fn has_intersection(&self, other: &Self) -> bool {
        (Orientation::calc(self.0, self.1, other.0).val()
            * Orientation::calc(self.0, self.1, other.1).val())
            <= 0.0
            && (Orientation::calc(other.0, other.1, self.0).val()
                * Orientation::calc(other.0, other.1, self.1).val())
                <= 0.0
    }

    /// 2つの線分の交点を求める
    ///
    /// **戻り値**
    /// - 2つの線分が点`x`で交わるとき → Some(`x`)
    /// - 2つの線分が交点を持たないとき → None
    pub fn cross_point(&self, other: &Self) -> Option<Point> {
        self.has_intersection(&other).then(|| {
            let l1 = self.to_line();
            let l2 = other.to_line();
            l1.cross_point(&l2)
        })
    }

    /// 点`p`と線分の距離を求める
    pub fn dist_point(&self, p: Point) -> f64 {
        let Self(a, b) = *self;

        if (b - a).dot(p - a) < EPS {
            return p.dist(a);
        }

        if (a - b).dot(p - b) < EPS {
            return p.dist(b);
        }

        self.to_line().dist_point(p)
    }

    /// 線分同士の距離を求める
    pub fn dist_segment(&self, other: &Self) -> f64 {
        if self.has_intersection(other) {
            return 0.0;
        }

        let mut res = f64::MAX;

        for d in [
            self.dist_point(other.0),
            self.dist_point(other.1),
            other.dist_point(self.0),
            other.dist_point(self.1),
        ] {
            if res > d {
                res = d;
            }
        }

        res
    }

    /// 直線に変換する
    pub fn to_line(&self) -> Line {
        let Self(a, b) = *self;
        Line(a, b)
    }
}
