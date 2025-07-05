"""RollingHashクラスのテストコード"""

import unittest
from lib.string.rolling_hash import (
    RollingHash,
)


class TestRollingHash(unittest.TestCase):
    """ローリングハッシュクラスのテスト"""

    def setUp(self):
        """テストの前に毎回実行される設定"""
        self.s1 = "abracadabra"
        self.rh1 = RollingHash(self.s1)

        self.s2 = "abcabcabc"
        self.rh2 = RollingHash(self.s2)

        self.s_empty = ""
        self.rh_empty = RollingHash(self.s_empty)

    def test_get_hash_basic(self):
        """get_hash の基本的なテスト"""
        # "abra" のハッシュ
        hash_abra = self.rh1.get_hash(0, 4)

        # "dabra" のハッシュ
        hash_dabra = self.rh1.get_hash(4, 9)
        self.assertNotEqual(hash_abra, hash_dabra)

        # "abracadabra" の全体ハッシュ
        full_hash = self.rh1.get_hash(0, self.rh1.n)
        self.assertEqual(full_hash, self.rh1.hash_[-1])

    def test_get_hash_same_substrings(self):
        """同じ部分文字列が同じハッシュ値を持つかのテスト"""
        # s1 における "abra" (0..4) と "abra" (7..11)
        hash1 = self.rh1.get_hash(0, 4)
        hash2 = self.rh1.get_hash(7, 11)
        self.assertEqual(hash1, hash2)

        # s2 における "abc" (0..3), "abc" (3..6), "abc" (6..9)
        h1 = self.rh2.get_hash(0, 3)
        h2 = self.rh2.get_hash(3, 6)
        h3 = self.rh2.get_hash(6, 9)
        self.assertEqual(h1, h2)
        self.assertEqual(h2, h3)

    def test_get_hash_edge_cases(self):
        """get_hash のエッジケース（空、単一文字）のテスト"""
        # 単一文字
        hash_b = self.rh1.get_hash(1, 2)
        expected_b = ord("b")
        self.assertEqual(hash_b, expected_b)  # 長さ1のハッシュはその文字のord値

        # 空文字列に対するハッシュ
        self.assertEqual(self.rh_empty.get_hash(0, 0), 0)

    def test_get_lcp(self):
        """get_lcp (最長共通接頭辞) のテスト"""
        # "abracadabra"
        # s[0:] = "abracadabra"
        # s[7:] = "abra"
        # LCP は "abra" の長さ 4
        self.assertEqual(self.rh1.get_lcp(0, 7), 4)

        # s[1:] = "bracadabra"
        # s[8:] = "bra"
        # LCP は "bra" の長さ 3
        self.assertEqual(self.rh1.get_lcp(1, 8), 3)

        # s[0:] = "abracadabra"
        # s[1:] = "bracadabra"
        # LCP はなし、長さ 0
        self.assertEqual(self.rh1.get_lcp(0, 1), 0)

        # s[2:] = "racadabra"
        # s[4:] = "cadabra"
        # LCP はなし、長さ 0
        self.assertEqual(self.rh1.get_lcp(2, 4), 0)

        # 同じ接頭辞同士の比較
        self.assertEqual(self.rh1.get_lcp(0, 0), self.rh1.n)

    def test_get_lcp_edge_cases(self):
        """get_lcp のエッジケースのテスト"""
        # 文字列の末尾での比較
        # s1 = "...a", s2 = "...ra"
        self.assertEqual(self.rh1.get_lcp(self.rh1.n - 1, self.rh1.n - 2), 0)

        # s1 = "...ra", s2 = "...abra"
        self.assertEqual(self.rh1.get_lcp(self.rh1.n - 2, self.rh1.n - 4), 0)

        # 範囲外のLCP
        self.assertEqual(self.rh1.get_lcp(self.rh1.n, self.rh1.n), 0)
        self.assertEqual(self.rh1.get_lcp(0, self.rh1.n), 0)

    def test_concat(self):
        """concat (ハッシュの連結) のテスト"""
        # s1 = "abra" + "cadabra"
        h1 = self.rh1.get_hash(0, 4)  # "abra"
        h2 = self.rh1.get_hash(4, 11)  # "cadabra"
        h2_len = 7

        concatenated_hash = self.rh1.concat(h1, h2, h2_len)
        full_hash = self.rh1.get_hash(0, 11)  # "abracadabra"

        self.assertEqual(concatenated_hash, full_hash)

        # s2 = "abc" + "abcabc"
        h1 = self.rh2.get_hash(0, 3)  # "abc"
        h2 = self.rh2.get_hash(3, 9)  # "abcabc"
        h2_len = 6

        concatenated_hash = self.rh2.concat(h1, h2, h2_len)
        full_hash = self.rh2.get_hash(0, 9)  # "abcabcabc"

        self.assertEqual(concatenated_hash, full_hash)

    def test_concat_with_empty(self):
        """空文字列との連結テスト"""
        full_hash = self.rh1.get_hash(0, self.rh1.n)
        empty_hash = self.rh1.get_hash(0, 0)

        # h + ""
        self.assertEqual(self.rh1.concat(full_hash, empty_hash, 0), full_hash)

        # "" + h
        self.assertEqual(self.rh1.concat(empty_hash, full_hash, self.rh1.n), full_hash)

    def test_random_base_generation(self):
        """基数を自動生成した場合の動作確認テスト（スモークテスト）"""
        # このテストは、エラーなくインスタンスが生成され、
        # 基本的な動作をすることをスモークテストとして確認する。
        try:
            s = "a_test_string_for_random_base"
            rh_random = RollingHash(s)  # base を指定しない

            # 簡単なハッシュ取得とLCPをチェック
            self.assertEqual(rh_random.get_hash(2, 6), rh_random.get_hash(2, 6))
            self.assertNotEqual(rh_random.get_hash(2, 6), rh_random.get_hash(3, 7))
            self.assertEqual(rh_random.get_lcp(0, 0), len(s))

        except Exception as e:
            self.fail(f"基数の自動生成で予期せぬエラーが発生しました: {e}")


if __name__ == "__main__":
    unittest.main(argv=["first-arg-is-ignored"], exit=False)
