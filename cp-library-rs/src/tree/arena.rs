//! ノードの確保を行うハンドラ

/// ノードを表すトレイト
pub trait ArenaNode {}

/// Storage とのやり取りを行うために用いるポインタ
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ptr(pub usize);

/// アリーナ構造のストレージ
#[derive(Default)]
pub struct Arena<N: ArenaNode> {
    nodes: Vec<N>,
    id: usize,
}

impl<N: ArenaNode> Arena<N> {
    /// アリーナ構造体のみ生成する
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            id: 0,
        }
    }

    /// n 個のノードを一気に確保する
    pub fn with_capacity(n: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(n),
            id: 0,
        }
    }

    /// 新たなノードを確保する
    pub fn alloc(&mut self, node: N) -> Ptr {
        let id = self.nodes.len();
        self.nodes.push(node);
        Ptr(id)
    }

    /// ノードの不変参照を取得する
    pub fn get(&self, ptr: Ptr) -> &N {
        &self.nodes[ptr.0]
    }

    /// ノードの可変参照を取得する
    pub fn get_mut(&mut self, ptr: Ptr) -> &mut N {
        &mut self.nodes[ptr.0]
    }
}

impl<N: ArenaNode + Default> Arena<N> {
    /// 空ノードを確保する
    pub fn alloc_default(&mut self) -> Ptr {
        self.alloc(N::default())
    }
}
