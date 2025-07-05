import unittest

from lib.data_structure.lazy_segment_tree import (
    LazySegmentTree,
    ADD_SUM,
    UPDATE_MIN,
    UPDATE_SUM,
)


class TestLazySegmentTree(unittest.TestCase):
    def test_add_sum(self):
        seg = LazySegmentTree(10, *ADD_SUM)

        # [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        self.assertEqual(seg.get_range(0, 10), 0)
        self.assertEqual(seg.get_range(0, 5), 0)
        self.assertEqual(seg.get_range(5, 10), 0)
        self.assertEqual(seg.get_range(3, 8), 0)

        seg.apply(0, 4, 2)
        # [2, 2, 2, 2, 0, 0, 0, 0, 0, 0]

        self.assertEqual(seg.get_range(0, 10), 8)
        self.assertEqual(seg.get_range(0, 5), 8)
        self.assertEqual(seg.get_range(5, 10), 0)
        self.assertEqual(seg.get_range(3, 8), 2)

        seg.apply(4, 10, 5)
        # [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        self.assertEqual(seg.get_range(0, 10), 38)
        self.assertEqual(seg.get_range(0, 5), 13)
        self.assertEqual(seg.get_range(5, 10), 25)
        self.assertEqual(seg.get_range(3, 8), 22)

        seg.apply(2, 6, -3)
        # [2, 2, -1, -1, 2, 2, 5, 5, 5, 5]

        self.assertEqual(seg.get_range(0, 10), 26)
        self.assertEqual(seg.get_range(0, 5), 4)
        self.assertEqual(seg.get_range(5, 10), 22)
        self.assertEqual(seg.get_range(3, 8), 13)

        seg.apply(8, 10, -10)
        # [2, 2, -1, -1, 2, 2, 5, 5, -5, -5]

        self.assertEqual(seg.get_range(0, 10), 6)
        self.assertEqual(seg.get_range(0, 5), 4)
        self.assertEqual(seg.get_range(5, 10), 2)
        self.assertEqual(seg.get_range(3, 8), 13)

    def test_UPDATE_MIN(self):
        INF = float("inf")
        seg = LazySegmentTree(10, *UPDATE_MIN)
        # [INF, INF, INF, INF, INF, INF, INF, INF, INF, INF]

        self.assertEqual(seg.get_range(0, 10), INF)
        self.assertEqual(seg.get_range(0, 5), INF)
        self.assertEqual(seg.get_range(5, 10), INF)
        self.assertEqual(seg.get_range(3, 8), INF)

        seg.apply(0, 4, 2)
        # [2, 2, 2, 2, INF, INF, INF, INF, INF, INF]

        self.assertEqual(seg.get_range(0, 10), 2)
        self.assertEqual(seg.get_range(0, 5), 2)
        self.assertEqual(seg.get_range(5, 10), INF)
        self.assertEqual(seg.get_range(3, 8), 2)

        seg.apply(4, 10, 5)
        # [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        self.assertEqual(seg.get_range(0, 10), 2)
        self.assertEqual(seg.get_range(0, 5), 2)
        self.assertEqual(seg.get_range(5, 10), 5)
        self.assertEqual(seg.get_range(3, 8), 2)

        seg.apply(2, 6, -3)
        # [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

        self.assertEqual(seg.get_range(0, 10), -3)
        self.assertEqual(seg.get_range(0, 5), -3)
        self.assertEqual(seg.get_range(5, 10), -3)
        self.assertEqual(seg.get_range(3, 8), -3)

        seg.apply(8, 10, -10)
        # [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

        self.assertEqual(seg.get_range(0, 10), -10)
        self.assertEqual(seg.get_range(0, 5), -3)
        self.assertEqual(seg.get_range(5, 10), -10)
        self.assertEqual(seg.get_range(3, 8), -3)

    def test_update_sum(self):
        seg = LazySegmentTree(10, *UPDATE_SUM)
        # [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        seg.apply(1, 4, 1)
        seg.apply(2, 5, -2)

        self.assertEqual(seg.get_range(0, 6), -5)
        self.assertEqual(seg.get_range(0, 2), 1)

        seg.apply(3, 6, 3)

        self.assertEqual(seg.get_range(3, 5), 6)
        self.assertEqual(seg.get_range(0, 6), 8)

    def test_from_array(self):
        seg = LazySegmentTree.from_array([1, 2, 3, 4, 5], *UPDATE_SUM)
        # [1, 2, 3, 4, 5]

        self.assertEqual(seg.get_range(0, 5), 15)
        self.assertEqual(seg.get_range(1, 4), 9)
        self.assertEqual(seg.get_range(2, 5), 12)

        seg.apply(1, 4, 2)
        # [1, 2, 2, 2, 5]

        self.assertEqual(seg.get_range(0, 5), 12)
        self.assertEqual(seg.get_range(1, 4), 6)
        self.assertEqual(seg.get_range(2, 5), 9)

        seg.apply(2, 5, -3)
        # [1, 2, -3, -3, -3]

        self.assertEqual(seg.get_range(0, 5), -6)
        self.assertEqual(seg.get_range(1, 4), -4)
        self.assertEqual(seg.get_range(2, 5), -9)

        seg.apply(0, 5, 10)
        # [10, 10, 10, 10, 10]

        self.assertEqual(seg.get_range(0, 5), 50)
        self.assertEqual(seg.get_range(1, 4), 30)
        self.assertEqual(seg.get_range(2, 5), 30)
