import unittest

from lib.graph.euler_tour import EulerTour


class TestEulerTour(unittest.TestCase):
    """EulerTourクラスのテスト"""

    def test_simple_tree(self):
        """単純な木構造のテスト"""
        n = 5
        edges = [(0, 1), (0, 2), (1, 3), (1, 4)]
        et = EulerTour(n)
        for u, v in edges:
            et.add_edge(u, v)
        et.build(0)

        # 期待される結果 (DFSの辺を辿る順序に依存するが、一つの妥当な例)
        # 0 -> 2 -> 1 -> 4 -> 3 という走査を仮定
        expected_in = [0, 3, 1, 6, 4]
        expected_out = [9, 8, 2, 7, 5]
        expected_depth = [0, 1, 1, 2, 2]

        # 走査順が正しいか
        self.assertEqual(et.in_, expected_in)
        self.assertEqual(et.out, expected_out)
        self.assertEqual(et.depth, expected_depth)

    def test_line_graph(self):
        """パスグラフ"""
        n = 4
        edges = [(0, 1), (1, 2), (2, 3)]
        et = EulerTour(n)
        for u, v in edges:
            et.add_edge(u, v)
        et.build(0)

        expected_in = [0, 1, 2, 3]
        expected_out = [7, 6, 5, 4]
        expected_depth = [0, 1, 2, 3]

        self.assertEqual(et.in_, expected_in)
        self.assertEqual(et.out, expected_out)
        self.assertEqual(et.depth, expected_depth)

    def test_root_is_leaf(self):
        """スターグラフ"""
        n = 4
        edges = [(0, 1), (0, 2), (0, 3)]
        et = EulerTour(n)
        for u, v in edges:
            et.add_edge(u, v)
        et.build(0)

        expected_in = [0, 5, 3, 1]
        expected_out = [7, 6, 4, 2]
        expected_depth = [0, 1, 1, 1]

        # 走査順によって [0, 1, 3, 2] もありえる
        self.assertEqual(et.in_, expected_in)
        self.assertEqual(et.out, expected_out)
        self.assertEqual(et.depth, expected_depth)

    def test_no_edges(self):
        """辺がないグラフのテスト"""
        n = 1
        et = EulerTour(n)
        et.build(0)

        # 頂点0のみ訪問
        self.assertEqual(et.in_, [0])
        self.assertEqual(et.out, [1])
        self.assertEqual(et.depth, [0])


if __name__ == "__main__":
    unittest.main()
