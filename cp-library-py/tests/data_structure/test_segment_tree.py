import unittest

from lib.data_structure.segment_tree import SegmentTree


class TestSegmentTree(unittest.TestCase):
    def test_sum(self):
        seg = SegmentTree.from_array([1, 2, 3, 4, 5], lambda: 0, lambda a, b: a + b)

        self.assertEqual(seg.get_range(0, 5), 15)
        self.assertEqual(seg.get_range(1, 4), 9)
        self.assertEqual(seg.get_range(2, 3), 3)
        seg.update(2, 10)
        self.assertEqual(seg.get_range(0, 5), 22)
        self.assertEqual(seg.get_range(1, 4), 16)
        self.assertEqual(seg.get_range(2, 3), 10)

    def test_min(self):
        seg = SegmentTree.from_array([1, 2, 3, 4, 5], lambda: 1e20, min)

        self.assertEqual(seg.get_range(0, 5), 1)
        self.assertEqual(seg.get_range(1, 4), 2)
        self.assertEqual(seg.get_range(2, 3), 3)
        seg.update(1, -1)
        self.assertEqual(seg.get_range(0, 5), -1)
        self.assertEqual(seg.get_range(1, 4), -1)
        self.assertEqual(seg.get_range(2, 3), 3)
