import unittest

from lib.graph.sccd import SCC


class TestSCC(unittest.TestCase):
    """SCCクラスのテスト"""

    def _assert_scc_results(
        self,
        scc: SCC,
        expected_components: list[list[int]],
        expected_dag_edges: set[tuple[int, int]],
    ):
        """
        SCCの結果を検証する補助関数。
        グループIDの割り当てが不定なため、正規化して比較する。
        """
        # 1. 強連結成分の数を検証
        self.assertEqual(scc.group_count, len(expected_components))

        # 2. 強連結成分の構成要素を検証 (順序を問わない)
        # 各成分の頂点リストをソートし、さらに成分のリスト全体をソートして比較
        actual_components_sorted = sorted([sorted(c) for c in scc.components])
        expected_components_sorted = sorted([sorted(c) for c in expected_components])
        self.assertListEqual(actual_components_sorted, expected_components_sorted)

        # 3. 縮約グラフ (DAG) を検証
        # まず、期待される成分リスト(ソート済)のインデックスを正規化されたグループIDとする。
        # 例: expected_components_sorted = [[0, 1], [2], [3]]
        # -> 正規化後グループID 0: [0, 1], 1: [2], 2: [3]

        # 実際の成分が、正規化後グループIDのどれに対応するかをマッピングする
        # `actual_components` は `scc.components` の各要素をソートしたもの
        actual_components = [sorted(c) for c in scc.components]

        # `component_to_normalized_gid` マップ:
        # { (0, 1): 0, (2,): 1, (3,): 2 } のような辞書を作成
        component_to_normalized_gid = {
            tuple(comp): i for i, comp in enumerate(expected_components_sorted)
        }

        # 実際のDAGの辺を、正規化されたグループIDのペアに変換する
        actual_dag_edges_normalized = set()
        for i in range(scc.group_count):
            # i 番目のグループの実際の頂点リスト (ソート済)
            from_comp_tuple = tuple(actual_components[i])
            # 対応する正規化後グループIDを取得
            from_gid_normalized = component_to_normalized_gid[from_comp_tuple]

            # i 番目のグループからのびる辺
            for j in scc.DAG[i]:
                to_comp_tuple = tuple(actual_components[j])
                to_gid_normalized = component_to_normalized_gid[to_comp_tuple]
                actual_dag_edges_normalized.add(
                    (from_gid_normalized, to_gid_normalized)
                )

        self.assertSetEqual(actual_dag_edges_normalized, expected_dag_edges)

    def test_atcoder_library_sample(self):
        """AtCoder Library のサンプルケース"""
        n = 6
        edges = [(1, 4), (5, 2), (3, 0), (0, 1), (1, 2), (2, 3), (3, 1)]
        scc = SCC(n)
        for u, v in edges:
            scc.add_edge(u, v)
        scc.decompose()

        # 期待される結果
        expected_components = [[0, 1, 2, 3], [4], [5]]
        # 正規化後グループID: 0:[0,1,2,3], 1:[4], 2:[5]
        # 辺 (1,4) は group 0 -> group 1
        # 辺 (5,2) は group 2 -> group 0
        expected_dag = {(0, 1), (2, 0)}

        self._assert_scc_results(scc, expected_components, expected_dag)

    def test_disconnected_graph(self):
        """非連結なグラフのケース"""
        n = 4
        edges = [(0, 1), (1, 0), (2, 3)]
        scc = SCC(n)
        for u, v in edges:
            scc.add_edge(u, v)
        scc.decompose()

        # 期待される結果
        expected_components = [[0, 1], [2], [3]]
        # 正規化後グループID: 0:[0,1], 1:[2], 2:[3]
        # 辺 (2,3) は group 1 -> group 2
        expected_dag = {(1, 2)}

        self._assert_scc_results(scc, expected_components, expected_dag)

    def test_no_edges(self):
        """辺がまったくないグラフのケース"""
        n = 4
        scc = SCC(n)
        scc.decompose()

        expected_components = [[0], [1], [2], [3]]
        expected_dag = set()

        self._assert_scc_results(scc, expected_components, expected_dag)

    def test_single_component(self):
        """全体が1つの強連結成分になるケース"""
        n = 3
        edges = [(0, 1), (1, 2), (2, 0)]
        scc = SCC(n)
        for u, v in edges:
            scc.add_edge(u, v)
        scc.decompose()

        expected_components = [[0, 1, 2]]
        expected_dag = set()

        self._assert_scc_results(scc, expected_components, expected_dag)

    def test_line_graph(self):
        """サイクルがなく、一直線に並んだグラフ (DAG) のケース"""
        n = 4
        edges = [(0, 1), (1, 2), (2, 3)]
        scc = SCC(n)
        for u, v in edges:
            scc.add_edge(u, v)
        scc.decompose()

        expected_components = [[0], [1], [2], [3]]
        # 正規化後グループID: 0:[0], 1:[1], 2:[2], 3:[3]
        expected_dag = {(0, 1), (1, 2), (2, 3)}

        self._assert_scc_results(scc, expected_components, expected_dag)

    def test_with_self_loop(self):
        """自己ループを持つグラフのケース"""
        n = 3
        edges = [(0, 0), (0, 1), (1, 2)]
        scc = SCC(n)
        for u, v in edges:
            scc.add_edge(u, v)
        scc.decompose()

        # 自己ループ (0,0) により 0 は単体で強連結成分
        expected_components = [[0], [1], [2]]
        # 正規化後グループID: 0:[0], 1:[1], 2:[2]
        expected_dag = {(0, 1), (1, 2)}

        self._assert_scc_results(scc, expected_components, expected_dag)
