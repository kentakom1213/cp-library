import unittest

from lib.number_theory.combination import Comb


class TestComb(unittest.TestCase):
    def setUp(self):
        self.comb_util = Comb(101)

    def test_comb(self):
        self.assertEqual(self.comb_util.comb(5, 2), 10)
        self.assertEqual(self.comb_util.comb(10, 0), 1)
        self.assertEqual(self.comb_util.comb(10, 10), 1)
        self.assertEqual(self.comb_util.comb(10, 3), 120)
        self.assertEqual(self.comb_util.comb(100, 2), 4950)
        # Edge cases
        self.assertEqual(self.comb_util.comb(4, 5), 0)
        self.assertEqual(self.comb_util.comb(5, -1), 0)

    def test_perm(self):
        self.assertEqual(self.comb_util.perm(5, 2), 20)
        self.assertEqual(self.comb_util.perm(10, 0), 1)
        self.assertEqual(self.comb_util.perm(10, 3), 720)
        self.assertEqual(self.comb_util.perm(100, 2), 9900)
        # Edge cases
        self.assertEqual(self.comb_util.perm(4, 5), 0)
        self.assertEqual(self.comb_util.perm(5, -1), 0)

    def test_fact(self):
        self.assertEqual(self.comb_util.fact(5), 120)
        self.assertEqual(self.comb_util.fact(0), 1)
        self.assertEqual(self.comb_util.fact(10), 3628800)
        # Edge cases
        self.assertEqual(self.comb_util.fact(-1), 0)

    def test_comb_rep(self):
        self.assertEqual(self.comb_util.comb_rep(3, 2), 6)
        self.assertEqual(self.comb_util.comb_rep(10, 3), 220)
        self.assertEqual(self.comb_util.comb_rep(5, 0), 1)
        # Edge cases
        self.assertEqual(self.comb_util.comb_rep(-1, 5), 0)
        self.assertEqual(self.comb_util.comb_rep(5, -1), 0)


if __name__ == "__main__":
    unittest.main()
