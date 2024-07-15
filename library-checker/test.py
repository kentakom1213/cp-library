from pathlib import Path
import subprocess
from concurrent.futures import ProcessPoolExecutor

CWD = Path(__file__).parent
TESTCASES = [f.name[:-3] for f in (CWD / "src/bin").glob("*.rs")]

TEMPLATE = """\
# Result

"""

# 5分で停止させる
KILL_TIME = 300


def run_test(problem: str) -> bool:
    """テストを実行する

    Args:
        problem (str): テスト対象の問題名

    Returns:
        Bool: 成功したかどうか
    """
    # テストケースの実行
    try:
        result = subprocess.run(
            [
                "cargo",
                "compete",
                "test",
                problem,
            ],
            cwd=CWD,
            timeout=KILL_TIME
        )

        # テストケースの結果を返す
        return result.returncode == 0
    except:
        # TIMEOUT
        return False


if __name__ == "__main__":

    # テストケースのダウンロード
    subprocess.run(
        [
            "cargo",
            "compete",
            "download",
            # "--full",
            # "--overwrite",
        ],
        cwd=CWD
    )

    # テストケースの実行
    with ProcessPoolExecutor() as executor:
        results = list(executor.map(run_test, TESTCASES))

    # テストケースの結果をTEMPLATEに追記
    for problem, result in sorted(zip(TESTCASES, results)):
        TEMPLATE += f"""\
- :{'white_check_mark' if result else 'x'}: [{problem}](https://judge.yosupo.jp/problem/{problem})
"""

    # 結果を出力
    (CWD / "README.md").write_text(TEMPLATE)
