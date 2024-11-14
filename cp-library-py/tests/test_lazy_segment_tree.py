import unittest

from cp_library_py.lazy_segment_tree import (
    LazySegmentTree,
    add_sum,
    update_min,
    update_sum,
)


class TestLazySegmentTree(unittest.TestCase):
    def test_add_sum(self):
        seg = LazySegmentTree(10, *add_sum)

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

    def test_update_min(self):
        INF = float('inf')
        seg = LazySegmentTree(10, *update_min)
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
        seg = LazySegmentTree(10, *update_sum)
        # [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        seg.apply(1, 4, 1)
        seg.apply(2, 5, -2)

        self.assertEqual(seg.get_range(0, 6), -5)
        self.assertEqual(seg.get_range(0, 2), 1)

        seg.apply(3, 6, 3)

        self.assertEqual(seg.get_range(3, 5), 6)
        self.assertEqual(seg.get_range(0, 6), 8)
