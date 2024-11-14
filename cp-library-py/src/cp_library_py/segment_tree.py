from typing import Callable, Generic, TypeVar

T = TypeVar("T")


class SegmentTree(Generic[T]):
    """セグメント木"""

    def __init__(self, n: int, id_: Callable[[], T], op: Callable[[T, T], T]) -> None:
        """セグメント木を初期化する

        Args:
            n (int): 要素数
            id_ (Callable[[], T]): 単位元を返す関数
            _op (Callable[[T, T], T]): 二項演算を行う関数
        """
        self._size = n
        self._offset = 1 << (n - 1).bit_length()
        self._id = id_
        self._op = op
        self._data = [id_() for _ in range(self._offset * 2)]

    @classmethod
    def from_array(cls, array: list[T], id_: Callable[[], T], op: Callable[[T, T], T]) -> "SegmentTree[T]":
        """配列からセグメント木を構築する

        Args:
            array (list[T]): 配列
            id_ (Callable[[], T]): 単位元を返す関数
            _op (Callable[[T, T], T]): 二項演算を行う関数

        Returns:
            SegmentTree[T]: 構築したセグメント木
        """
        seg = cls(len(array), id_, op)
        seg._data[seg._offset: seg._offset + len(array)] = array

        for i in range(seg._offset - 1, 0, -1):
            seg._data[i] = seg._op(seg._data[i * 2], seg._data[i * 2 + 1])

        return seg

    def update(self, i: int, x: T) -> None:
        """i 番目の要素を x に更新する

        Args:
            i (int): 更新する要素のインデックス
            x (T): 更新する値
        """
        i += self._offset
        self._data[i] = x
        while i > 1:
            i >>= 1
            self._data[i] = self._op(self._data[i * 2], self._data[i * 2 + 1])

    def get_range(self, l: int, r: int) -> T:
        """区間 [l,r) の演算結果を返す

        Args:
            l (int): 区間の左端
            r (int): 区間の右端

        Returns:
            T: 演算結果
        """
        l += self._offset
        r += self._offset
        res_l = self._id()
        res_r = self._id()

        while l < r:
            if l & 1:
                res_l = self._op(res_l, self._data[l])
                l += 1
            if r & 1:
                r -= 1
                res_r = self._op(self._data[r], res_r)
            l >>= 1
            r >>= 1

        return self._op(res_l, res_r)

    def __getitem__(self, i: int) -> T:
        """i 番目の要素を取得する

        Args:
            i (int): 取得する要素のインデックス

        Returns:
            T: 取得した要素
        """
        return self._data[i + self._offset]

    def __setitem__(self, i: int, x: T) -> None:
        """i 番目の要素を x に更新する

        Args:
            i (int): 更新する要素のインデックス
            x (T): 更新する値
        """
        self.update(i, x)
