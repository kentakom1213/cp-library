"""組合せの前計算"""

# MOD = 1_000_000_007
MOD = 998_244_353


class Comb:
    def __init__(self, max_size: int):
        fac = [1] * max_size
        finv = [1] * max_size
        inv = [1] * max_size

        for i in range(2, max_size):
            fac[i] = fac[i - 1] * i % MOD
            inv[i] = -(MOD // i) * inv[MOD % i] % MOD
            finv[i] = finv[i - 1] * inv[i]

        # 変数に格納
        self._fac = fac
        self._finv = finv

    def comb(self, n: int, r: int) -> int:
        """n個のものからr個を選ぶ組合せの数を計算する"""
        if n < r or r < 0:
            return 0
        return self._fac[n] * self._finv[r] * self._finv[n - r] % MOD

    def perm(self, n: int, r: int) -> int:
        """n個のものからr個を選ぶ順列の数を計算する"""
        if n < r or r < 0:
            return 0
        return self._fac[n] * self._finv[n - r] % MOD

    def fact(self, n: int) -> int:
        """n個のものの順列の数を計算する"""
        if n < 0:
            return 0
        return self._fac[n]

    def inv(self, n: int) -> int:
        """nの逆元を計算する"""
        if n < 0:
            return 0
        return self._finv[n]

    def comb_rep(self, n: int, r: int) -> int:
        """n個のものからr個を重複を許して選ぶ組合せの数を計算する"""
        if n < 0 or r < 0:
            return 0
        return self.comb(n + r - 1, r)
