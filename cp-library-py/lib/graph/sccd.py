"""強連結成分分解"""

import sys

sys.setrecursionlimit(1000000)


class SCC:
    def __init__(self, n: int):
        """頂点数 n の空グラフを構築する．"""

        self.n = n
        """頂点数"""
        self.G: list[list[int]] = [[] for _ in range(n)]
        """グラフ"""
        self.rG: list[list[int]] = [[] for _ in range(n)]
        """辺の向きを反転させたグラフ"""
        self.group_count: int | None = None
        """強連結成分の数"""
        self.components: list[int] | None = None
        """強連結成分の集合"""
        self.belong: list[list[int]] | None = None
        """頂点 i がどのグループに所属しているか"""
        self.DAG: list[list[int]] | None = None
        """構成されたDAG"""

    def add_edge(self, u: int, v: int):
        """グラフに辺 (u,v) を追加する"""
        self.G[u].append(v)
        self.rG[v].append(u)

    def decompose(self):
        """強連結成分分解を行う"""
        # 帰りがけ順で順序付ける
        order = []
        visited = [False] * self.n
        for u in range(self.n):
            SCC.__dfs(u, self.G, order, visited)

        # 強連結成分に分解
        group = 0
        belong = [-1] * self.n
        for u in reversed(order):
            if belong[u] == -1:
                SCC.__rdfs(u, group, self.rG, belong)
                group += 1

        # DAG を構成
        DAG = [[] for _ in range(group)]
        for u in range(self.n):
            for v in self.G[u]:
                x = belong[u]
                y = belong[v]
                if x != y:
                    DAG[x].append(y)

        # 成分を構成
        components = [[] for _ in range(group)]
        for u, g in enumerate(belong):
            components[g].append(u)

        self.group_count = group
        self.belong = belong
        self.DAG = DAG
        self.components = components

    @staticmethod
    def __dfs(u: int, G: list[list[int]], order: list[int], visited: list[int]):
        """頂点 u からの帰りがけ順での到達順序を調べる"""
        if visited[u]:
            return
        visited[u] = True
        for v in G[u]:
            SCC.__dfs(v, G, order, visited)
        order.append(u)

    @staticmethod
    def __rdfs(u: int, group: int, rG: list[list[int]], belong: list[int]):
        """逆辺を張ったグラフで u から到達できる頂点をグループ group で塗り分ける"""
        if belong[u] != -1:
            return
        belong[u] = group
        for v in rG[u]:
            SCC.__rdfs(v, group, rG, belong)
