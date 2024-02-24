//! 作用付きモノイド

/// 作用付きモノイド
pub trait ExtMonoid {
    /// 要素のデータ型
    type X: Clone + PartialEq;
    /// 作用素のデータ型
    type M: Clone + PartialEq;
    /// 要素Xの単位元
    const IX: Self::X;
    /// 作用素Mの単位元
    const IM: Self::M;
    /// 要素同士の演算
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X;
    /// 要素に対する作用
    fn apply(x: &Self::X, y: &Self::M) -> Self::X;
    /// 作用素同士の演算
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M;
    /// 作用素の集約
    fn aggregate(x: &Self::M, p: usize) -> Self::M;
}
