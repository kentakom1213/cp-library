"""ベクトル演算の実装"""


class Vec2:
    """2次元ベクトルを表すクラス
    Attributes:
        x (float): x座標
        y (float): y座標
    """

    def __init__(self, x: float, y: float):
        self._x = x
        self._y = y

    def x(self) -> float:
        return self._x

    def y(self) -> float:
        return self._y

    def norm2(self) -> float:
        """ベクトルのノルムの二乗を返す"""
        return self._x * self._x + self._y * self._y

    def norm(self) -> float:
        """ベクトルのノルムを返す"""
        return self.norm2() ** 0.5

    def normalize(self) -> "Vec2":
        """ベクトルを正規化する"""
        norm = self.norm()
        if norm == 0:
            raise ValueError("Cannot normalize a zero vector")
        return Vec2(x=self._x / norm, y=self._y / norm)

    def dot(self, other: "Vec2") -> float:
        """ドット積を計算する"""
        return self._x * other._x + self._y * other._y

    def cross(self, other: "Vec2") -> float:
        """外積を計算する（2Dではスカラー値）"""
        return self._x * other._y - self._y * other._x

    def __hash__(self) -> int:
        return hash((self._x, self._y))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Vec2):
            return NotImplemented
        return self._x == other._x and self._y == other._y

    def __repr__(self) -> str:
        return f"Vec2(x={self._x}, y={self._y})"

    def __mul__(self, scalar: float) -> "Vec2":
        """スカラー倍を行う"""
        return Vec2(x=self._x * scalar, y=self._y * scalar)

    def __truediv__(self, scalar: float) -> "Vec2":
        """スカラー除算を行う"""
        if scalar == 0:
            raise ValueError("Cannot divide by zero")
        return Vec2(x=self._x / scalar, y=self._y / scalar)

    def __add__(self, other: "Vec2") -> "Vec2":
        return Vec2(x=self._x + other._x, y=self._y + other._y)

    def __sub__(self, other: "Vec2") -> "Vec2":
        return Vec2(x=self._x - other._x, y=self._y - other._y)
