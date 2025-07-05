import unittest

from lib.geometry.vec2 import Vec2


class TestVec2(unittest.TestCase):
    def test_init_and_properties(self):
        v = Vec2(3, 4)
        self.assertEqual(v.x(), 3)
        self.assertEqual(v.y(), 4)

    def test_eq(self):
        v1 = Vec2(1, 2)
        v2 = Vec2(1, 2)
        v3 = Vec2(3, 4)
        self.assertEqual(v1, v2)
        self.assertNotEqual(v1, v3)
        self.assertNotEqual(v1, (1, 2))

    def test_repr(self):
        v = Vec2(3, 4)
        self.assertEqual(repr(v), "Vec2(x=3, y=4)")

    def test_add(self):
        v1 = Vec2(1, 2)
        v2 = Vec2(3, 4)
        self.assertEqual(v1 + v2, Vec2(4, 6))

    def test_sub(self):
        v1 = Vec2(4, 6)
        v2 = Vec2(3, 4)
        self.assertEqual(v1 - v2, Vec2(1, 2))

    def test_mul(self):
        v = Vec2(3, 4)
        self.assertEqual(v * 2, Vec2(6, 8))

    def test_truediv(self):
        v = Vec2(6, 8)
        self.assertEqual(v / 2, Vec2(3, 4))
        with self.assertRaises(ValueError):
            v / 0

    def test_norm2(self):
        v = Vec2(3, 4)
        self.assertEqual(v.norm2(), 25)

    def test_norm(self):
        v = Vec2(3, 4)
        self.assertEqual(v.norm(), 5)

    def test_normalize(self):
        v = Vec2(3, 4)
        normalized_v = v.normalize()
        self.assertAlmostEqual(normalized_v.x(), 0.6)
        self.assertAlmostEqual(normalized_v.y(), 0.8)
        self.assertAlmostEqual(normalized_v.norm(), 1.0)

        zero_v = Vec2(0, 0)
        with self.assertRaises(ValueError):
            zero_v.normalize()

    def test_dot(self):
        v1 = Vec2(1, 2)
        v2 = Vec2(3, 4)
        self.assertEqual(v1.dot(v2), 11)

    def test_cross(self):
        v1 = Vec2(1, 2)
        v2 = Vec2(3, 4)
        self.assertEqual(v1.cross(v2), -2)

    def test_hash(self):
        v1 = Vec2(3, 4)
        v2 = Vec2(3, 4)
        v3 = Vec2(1, 2)
        s = {v1, v2, v3}
        self.assertEqual(len(s), 2)
        self.assertIn(v1, s)
        self.assertIn(Vec2(3, 4), s)
        self.assertNotIn(Vec2(4, 3), s)
