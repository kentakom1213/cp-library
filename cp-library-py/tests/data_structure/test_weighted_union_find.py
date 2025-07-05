import unittest

from lib.data_structure.weighted_union_find import (
    WeightedUnionFind,
    ADD_INT_GROUP,
    XOR_INT_GROUP,
)


class TestWeightedUnionFind(unittest.TestCase):
    def test_add_int_group(self):
        wuf = WeightedUnionFind(5, *ADD_INT_GROUP)

        # 初期状態
        self.assertEqual(wuf.group_count(), 5)
        for i in range(5):
            self.assertEqual(wuf.size(i), 1)
            self.assertEqual(wuf.get_weight(i), 0)

        # 0 -> 1 (weight(1) - weight(0) = 1)
        self.assertTrue(wuf.unite(0, 1, 1))
        self.assertTrue(wuf.is_same(0, 1))
        self.assertFalse(wuf.is_same(0, 2))
        self.assertEqual(wuf.diff(0, 1), 1)
        self.assertEqual(wuf.diff(1, 0), -1)
        self.assertEqual(wuf.group_count(), 4)
        self.assertEqual(wuf.size(0), 2)
        self.assertEqual(wuf.size(1), 2)

        # 1 -> 2 (weight(2) - weight(1) = 2)
        self.assertTrue(wuf.unite(1, 2, 2))
        self.assertTrue(wuf.is_same(0, 2))
        self.assertEqual(wuf.diff(0, 2), 3)  # 1 + 2
        self.assertEqual(wuf.diff(2, 0), -3)
        self.assertEqual(wuf.group_count(), 3)
        self.assertEqual(wuf.size(2), 3)

        # 3 -> 4 (weight(4) - weight(3) = 5)
        self.assertTrue(wuf.unite(3, 4, 5))
        self.assertTrue(wuf.is_same(3, 4))
        self.assertFalse(wuf.is_same(0, 3))
        self.assertEqual(wuf.diff(3, 4), 5)
        self.assertEqual(wuf.group_count(), 2)

        # 0 -> 3 (weight(3) - weight(0) = 10)
        self.assertTrue(wuf.unite(0, 3, 10))
        self.assertTrue(wuf.is_same(0, 4))
        self.assertEqual(wuf.diff(0, 4), 15)  # 10 + 5
        self.assertEqual(wuf.diff(1, 4), 14)  # -1 + 10 + 5
        self.assertEqual(wuf.diff(2, 4), 12)  # -2 -1 + 10 + 5
        self.assertEqual(wuf.group_count(), 1)
        self.assertEqual(wuf.size(0), 5)

        # 矛盾した併合
        self.assertFalse(wuf.unite(0, 2, 0)) # diff(0, 2) is 3
        self.assertTrue(wuf.unite(0, 2, 3)) # diff(0, 2) is 3

        # diff on different groups
        wuf_sep = WeightedUnionFind(4, *ADD_INT_GROUP)
        wuf_sep.unite(0, 1, 1)
        wuf_sep.unite(2, 3, 2)
        self.assertIsNone(wuf_sep.diff(0, 2))


    def test_xor_int_group(self):
        wuf = WeightedUnionFind(5, *XOR_INT_GROUP)

        # 0 -> 1 (weight(1) ^ weight(0) = 1)
        self.assertTrue(wuf.unite(0, 1, 1))
        self.assertEqual(wuf.diff(0, 1), 1)
        self.assertEqual(wuf.diff(1, 0), 1)

        # 1 -> 2 (weight(2) ^ weight(1) = 2)
        self.assertTrue(wuf.unite(1, 2, 2))
        self.assertEqual(wuf.diff(0, 2), 3)  # 1 ^ 2

        # 3 -> 4 (weight(4) ^ weight(3) = 5)
        self.assertTrue(wuf.unite(3, 4, 5))
        self.assertEqual(wuf.diff(3, 4), 5)

        # 0 -> 3 (weight(3) ^ weight(0) = 10)
        self.assertTrue(wuf.unite(0, 3, 10))
        self.assertEqual(wuf.diff(0, 4), 15) # 10 ^ 5
        self.assertEqual(wuf.diff(1, 4), 14) # 1 ^ 10 ^ 5
        self.assertEqual(wuf.diff(2, 4), 12) # 2 ^ 1 ^ 10 ^ 5

        # 矛盾した併合
        self.assertFalse(wuf.unite(0, 2, 0)) # diff(0, 2) is 3
        self.assertTrue(wuf.unite(0, 2, 3)) # diff(0, 2) is 3

    def test_invalid_init(self):
        with self.assertRaises(ValueError):
            WeightedUnionFind(-1, *ADD_INT_GROUP)


if __name__ == "__main__":
    unittest.main()
