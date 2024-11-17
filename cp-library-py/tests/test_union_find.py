import unittest

from cp_library_py.union_find import UnionFind


class TestUnionFind(unittest.TestCase):
    def test_union_find(self):
        uf = UnionFind(8)
        """
         0 1 2 3 4 5 6 7
        """

        uf.unite(1, 2)
        uf.unite(3, 2)
        """
         0 1-2-3 4 5 6 7
        """

        self.assertEqual(uf.group_count(), 6)

        self.assertTrue(uf.is_same(1, 3))
        self.assertFalse(uf.is_same(1, 4))
        self.assertEqual(uf.size(1), 3)

        uf.unite(2, 4)
        """
         0 1-2-3-4 5 6 7
        """

        self.assertEqual(uf.group_count(), 5)
        self.assertTrue(uf.is_same(4, 1))
        self.assertEqual(uf.size(4), 4)

        uf.unite(4, 2)
        uf.unite(0, 0)
        """
         0 1-2-3-4 5 6 7
        """

        self.assertEqual(uf.group_count(), 5)
        self.assertTrue(uf.is_same(0, 0))

        uf.unite(0, 7)
        """
         0 1-2-3-4 5 6 7
         └─────────────┘
        """

        self.assertEqual(uf.group_count(), 4)
        self.assertTrue(uf.is_same(0, 7))

        uf.unite(5, 6)
        """
         0 1-2-3-4 5-6 7
         └─────────────┘
        """

        self.assertEqual(uf.group_count(), 3)
        self.assertTrue(uf.is_same(5, 6))
        self.assertFalse(uf.is_same(5, 7))

        uf.unite(4, 5)
        uf.unite(6, 7)
        """
         0-1-2-3-4-5-6-7
        """

        self.assertEqual(uf.group_count(), 1)

        uf.unite(0, 7)
        """
         0-1-2-3-4-5-6-7
        """

        self.assertEqual(uf.group_count(), 1)
