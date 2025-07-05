from typing import TypeVar, Generic, Callable, List, Optional

T = TypeVar("T")


class WeightedUnionFind(Generic[T]):
    """重み付きUnion-Find (ポテンシャル付きUnion-Find)

    2つの要素 x, y を、重みの差が w (weight(y) - weight(x) = w) となるように
    併合することができるデータ構造。アーベル群をなす演算であれば、どのような
    型の重みでも扱うことが可能。
    """

    def __init__(
        self,
        n: int,
        op: Callable[[T, T], T],
        identity_func: Callable[[], T],
        inverse: Callable[[T], T],
    ):
        """WeightedUnionFindを初期化する。

        Args:
            n (int): 要素数
            op (Callable[[T, T], T]): 重みに使う二項演算 (例: `lambda a, b: a + b`)
            identity_func (Callable[[], T]): 重みの単位元を返す関数 (例: `lambda: 0`)
            inverse (Callable[[T], T]): 重みの逆元を計算する関数 (例: `lambda a: -a`)
        """
        if n < 0:
            raise ValueError("Number of elements must be non-negative.")

        self.n: int = n
        self.par: List[int] = list(range(n))
        self.rank: List[int] = [1] * n
        self.weight: List[T] = [identity_func() for _ in range(n)]

        self.op: Callable[[T, T], T] = op
        self.inverse: Callable[[T], T] = inverse

        self._group_count: int = n

    def find(self, x: int) -> int:
        """要素 x の根を求め、経路圧縮を行う。

        Note:
            この実装は再帰を使用しているため、要素数が多い場合は
            Pythonの再帰深度上限に達する可能性がある。
        """
        if self.par[x] == x:
            return x

        root = self.find(self.par[x])
        self.weight[x] = self.op(self.weight[x], self.weight[self.par[x]])
        self.par[x] = root
        return root

    def get_weight(self, x: int) -> T:
        """要素 x のポテンシャル（根からの重み）を返す。"""
        self.find(x)
        return self.weight[x]

    def is_same(self, x: int, y: int) -> bool:
        """要素 x と y が同じ集合に属するか判定する。"""
        return self.find(x) == self.find(y)

    def unite(self, x: int, y: int, w: T) -> bool:
        """要素 x と y を、weight(y) - weight(x) = w となるように併合する。"""
        if self.is_same(x, y):
            actual_diff = self.diff(x, y)
            return actual_diff == w

        root_x = self.find(x)
        root_y = self.find(y)

        w_edge = self.op(w, self.get_weight(x))
        w_edge = self.op(w_edge, self.inverse(self.get_weight(y)))

        if self.rank[root_x] < self.rank[root_y]:
            root_x, root_y = root_y, root_x
            w_edge = self.inverse(w_edge)

        self.par[root_y] = root_x
        self.rank[root_x] += self.rank[root_y]
        self.weight[root_y] = w_edge
        self._group_count -= 1
        return True

    def diff(self, x: int, y: int) -> Optional[T]:
        """要素 x と y の重みの差 (weight(y) - weight(x)) を計算する。"""
        if not self.is_same(x, y):
            return None

        w_y = self.get_weight(y)
        inv_w_x = self.inverse(self.get_weight(x))
        return self.op(w_y, inv_w_x)

    def size(self, x: int) -> int:
        """要素 x が属する集合の要素数を返す。"""
        return self.rank[self.find(x)]

    def group_count(self) -> int:
        """連結成分の数を返す。"""
        return self._group_count


# === 関数例 ===
ADD_INT_GROUP = (lambda a, b: a + b, lambda: 0, lambda a: -a)

XOR_INT_GROUP = (lambda a, b: a ^ b, lambda: 0, lambda a: a)
