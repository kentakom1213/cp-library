from typing import Callable, Generic, TypeVar

T = TypeVar("T")
F = TypeVar("F")


class LazySegmentTree(Generic[T, F]):
    """遅延セグメント木"""

    def __init__(
        self,
        n: int,
        id_t: Callable[[], T],
        id_f: Callable[[], F],
        op: Callable[[T, T], T],
        mapping: Callable[[T, F], T],
        composition: Callable[[F, F], F],
        aggregation: Callable[[F, int], F],
    ) -> None:
        """遅延セグメント木を初期化する

        Args:
            n (int): 木のサイズ
            id_t (Callable[[], T]): 値の単位元
            id_f (Callable[[], F]): 作用の単位元
            op (Callable[[T, T], T]): 二項演算
            mapping (Callable[[T, F], T]): 作用の適用
            composition (Callable[[F, F], F]): 作用の合成
            aggregation (Callable[[F, int], F]): 作用の繰り返し
        """
        # 定数
        self._size = n
        self._offset = 1 << (n - 1).bit_length()

        # 関数
        self._id_t = id_t
        self._id_f = id_f
        self._op = op
        self._mapping = mapping
        self._composition = composition
        self._aggregation = aggregation

        # データ
        self._data = [id_t() for _ in range(self._offset * 2)]
        self._lazy = [id_f() for _ in range(self._offset * 2)]

    def __eval(self, i: int, len: int):
        """遅延評価を行う

        Args:
            i (int): 遅延評価を行うノードのインデックス
            len (int): 遅延評価を行うノードの長さ
        """
        if self._lazy[i] == self._id_f():
            return

        # 葉でなければ子に伝搬
        if i < self._offset:
            self._lazy[i * 2] = self._composition(
                self._lazy[i * 2],
                self._lazy[i]
            )
            self._lazy[i * 2 + 1] = self._composition(
                self._lazy[i * 2 + 1],
                self._lazy[i]
            )

        # 値の更新
        self._data[i] = self._mapping(
            self._data[i],
            self._aggregation(self._lazy[i], len)
        )
        self._lazy[i] = self._id_f()

    def apply(
        self,
        left: int,
        right: int,
        val: F,
        begin: int = 0,
        end: int | None = None,
        idx: int = 1
    ):
        """区間 [l, r) に作用を適用する

        Args:
            left (int): 区間の左端
            right (int): 区間の右端
            val (F): 作用
        """
        if end is None:
            end = self._offset

        # 遅延値を評価
        self.__eval(idx, end - begin)

        # 区間を内包するとき
        if left <= begin and end <= right:
            self._lazy[idx] = self._composition(self._lazy[idx], val)
            self.__eval(idx, end - begin)

        # 区間が重なるとき
        elif left < end and begin < right:
            mid = (begin + end) // 2
            # 左の子を更新
            self.apply(left, right, val, begin, mid, idx * 2)
            # 右の子を更新
            self.apply(left, right, val, mid, end, idx * 2 + 1)
            # 値を更新
            self._data[idx] = \
                self._op(self._data[idx * 2], self._data[idx * 2 + 1])

    def get_range(
        self,
        left: int,
        right: int,
        begin: int = 0,
        end: int | None = None,
        idx: int = 1
    ):
        """区間 [l, r) の値を取得する

        Args:
            left (int): 区間の左端
            right (int): 区間の右端
        """
        if end is None:
            end = self._offset

        # 遅延値を評価
        self.__eval(idx, end - begin)

        # 区間が交差しないとき
        if end <= left or right <= begin:
            return self._id_t()

        # 区間を内包するとき
        if left <= begin and end <= right:
            return self._data[idx]

        # 区間が重なるとき
        mid = (begin + end) // 2
        left_val = self.get_range(left, right, begin, mid, idx * 2)
        right_val = self.get_range(left, right, mid, end, idx * 2 + 1)
        return self._op(left_val, right_val)


# 関数例
add_sum = (
    # id_t
    lambda: 0,
    # id_f
    lambda: 0,
    # op
    lambda x, y: x + y,
    # mapping
    lambda x, f: x + f,
    # composition
    lambda f, g: f + g,
    # aggregation
    lambda f, n: f * n
)

update_min = (
    # id_t
    lambda: float("inf"),
    # id_f
    lambda: float("inf"),
    # op
    min,
    # mapping
    lambda _x, f: f,
    # composition
    lambda _f, g: g,
    # aggregation
    lambda f, _n: f
)

update_max = (
    # id_t
    lambda: float("-inf"),
    # id_f
    lambda: float("-inf"),
    # op
    max,
    # mapping
    lambda _x, f: f,
    # composition
    lambda _f, g: g,
    # aggregation
    lambda f, _n: f
)

add_min = (
    # id_t
    lambda: float("inf"),
    # id_f
    lambda: 0,
    # op
    min,
    # mapping
    lambda x, f: x + f,
    # composition
    lambda f, g: f + g,
    # aggregation
    lambda f, _n: f
)

update_sum = (
    # id_t
    lambda: 0,
    # id_f
    lambda: None,
    # op
    lambda x, y: x + y,
    # mapping
    lambda _x, f: f,
    # composition
    lambda _f, g: g,
    # aggregation
    lambda f, n: f * n if f is not None else None
)
