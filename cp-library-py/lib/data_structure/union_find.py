class UnionFind:
    def __init__(self, n: int) -> None:
        self._n = n

        # self._data[i] :=
        #     -x → i 番目の要素が根であり、グループに属する頂点数が x 個
        #      x → 要素 i の親が x
        self._data = [-1] * n

        # グループの個数
        self._group_count = n

    def find(self, u: int) -> int:
        """要素 u が属するグループの根を返す

        Args:
            u (int): 要素の番号

        Returns:
            int: 要素 u が属するグループの根
        """
        root = u

        while self._data[root] >= 0:
            root = self._data[root]

        while u != root:
            self._data[u], u = root, self._data[u]

        return root

    def is_same(self, u: int, v: int) -> bool:
        """要素 u と要素 v が同じグループに属するかを返す

        Args:
            u (int): 要素の番号
            v (int): 要素の番号

        Returns:
            bool: 要素 u と要素 v が同じグループに属するか
        """
        return self.find(u) == self.find(v)

    def unite(self, u: int, v: int) -> bool:
        """要素 u が属するグループと要素 v が属するグループを併合する

        Args:
            u (int): 要素の番号
            v (int): 要素の番号

        Returns:
            bool: 併合が行われたか
        """
        u = self.find(u)
        v = self.find(v)

        if u == v:
            return False

        if self._data[u] > self._data[v]:
            u, v = v, u

        self._data[u] += self._data[v]
        self._data[v] = u

        self._group_count -= 1

        return True

    def size(self, u: int) -> int:
        """要素 u が属するグループの頂点数を返す

        Args:
            u (int): 要素の番号

        Returns:
            int: 要素 u が属するグループの頂点数
        """
        return -self._data[self.find(u)]

    def group_count(self) -> int:
        """グループの個数を返す

        Returns:
            int: グループの個数
        """
        return self._group_count
