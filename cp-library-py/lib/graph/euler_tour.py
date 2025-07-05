"""オイラーツアー"""


class EulerTour:
    def __init__(self, n: int):
        """頂点数 n で初期化する"""

        self.n = n
        self.G: list[list[int]] = [[] for _ in range(n)]
        self.in_: list[int] = []
        self.out: list[int] = []
        self.depth: list[int] = []

    def add_edge(self, u: int, v: int):
        """辺 {u,v} を追加する"""

        self.G[u].append(v)
        self.G[v].append(u)

    def build(self, root: int):
        """root を根として順序付けを行う"""

        in_ = [-1] * self.n
        out = [-1] * self.n
        depth = [-1] * self.n

        d, i = 0, 0
        stack = [root]

        while stack:
            u = stack.pop()
            if depth[u] == -1:
                stack.append(u)
                # 行きがけ処理
                in_[u] = i
                depth[u] = d
                i += 1
                d += 1
                for v in self.G[u]:
                    if depth[v] != -1:
                        continue
                    stack.append(v)
            else:
                # 帰りがけ処理
                d -= 1
                out[u] = i
                i += 1

        self.in_ = in_
        self.out = out
        self.depth = depth

    def get_id(self, v: int) -> tuple[int, int]:
        """頂点 v の (行きがけ順, 帰りがけ順) の番号を返す"""
        return (self.in_[v], self.out[v])
