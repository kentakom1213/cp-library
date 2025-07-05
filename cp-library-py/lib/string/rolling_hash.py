"""ローリングハッシュ"""

import random

MOD: int = (1 << 61) - 1
__FACTORS: dict[int, int] = {
    2: 1,
    3: 2,
    5: 2,
    7: 1,
    11: 1,
    13: 1,
    31: 1,
    41: 1,
    61: 1,
    151: 1,
    331: 1,
    1321: 1,
}
"""MOD-1 の素因数分解"""


def _is_primitive_root(a: int) -> bool:
    """a が F_MOD の原始根であるか判定する"""

    return all(pow(a, (MOD - 1) // pi, MOD) != 1 for pi in __FACTORS)


def _generate_base() -> int:
    """MOD における原始根を生成する"""
    while True:
        b = random.randint(1, MOD)
        if _is_primitive_root(b):
            return b


class RollingHash:
    """ローリングハッシュ"""

    def __init__(self, s: str, base: int | None = None):
        """文字列 s のローリングハッシュを生成する"""
        n = len(s)
        power = [1] * (n + 1)
        hash_ = [0] * (n + 1)

        # 原始根の生成
        if base is None:
            base = _generate_base()

        # ハッシュの計算
        h, p = 0, 1
        for i, c in enumerate(s):
            h = (h * base + ord(c)) % MOD
            p = (p * base) % MOD
            hash_[i + 1] = h
            power[i + 1] = p

        self.n = n
        self.base = base
        self.power = power
        self.hash_ = hash_

    def get_hash(self, l: int, r: int) -> int:
        """区間 [l, r) のハッシュを取得する"""
        return (self.hash_[r] - self.hash_[l] * self.power[r - l]) % MOD

    def get_lcp(self, a: int, b: int) -> int:
        """文字列 s[a:] と s[b:] の最長共通接頭辞を求める"""
        length = max(self.n - max(a, b), 0)
        lo, hi = 0, length + 1
        while hi - lo > 1:
            mid = (lo + hi) // 2
            if self.get_hash(a, a + mid) == self.get_hash(b, b + mid):
                lo = mid
            else:
                hi = mid
        return lo

    def concat(self, h1: int, h2: int, h2_len: int) -> int:
        """ハッシュ h1, h2 (長さ h2_len) を連結した文字列のハッシュを求める"""
        return (h1 * self.power[h2_len] + h2) % MOD
