import json
import random
import string
import statistics
import subprocess
from collections import defaultdict

from tqdm import tqdm

# Config
BOARD_COUNT = 100
SOLVER_THREADS = 1


def generate_board():
    return ''.join(random.choices(string.ascii_lowercase, k=25))


def run_solver(swaps, board):
    cmd = [
        "./target/release/spellcast-solver",
        "-t", str(SOLVER_THREADS),
        "-s", str(swaps),
        "-b", board,
        "-f", "json",
    ]
    sub = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(sub.stdout)['elapsed_ms']


def main():
    print(f"Running benchmark for {BOARD_COUNT} boards")
    times = defaultdict(list)
    dict_times = []

    for i in tqdm(range(BOARD_COUNT)):
        i_swaps = i % 4
        time = run_solver(i_swaps, generate_board())
        times[i_swaps].append(time['solver'])
        dict_times.append(time['dict'])

    print("Benchmark done")
    print("[X swaps] min/avg/max/mdev")

    for swaps in range(4):
        if times[swaps]:
            timex = times[swaps]
            mean = statistics.mean(timex)
            mdev = sum(abs(mean - time) for time in timex) / len(timex)
            print(f"[{swaps} swaps] {min(timex):.1f}/{mean:.1f}/{max(timex):.1f}/{mdev:.1f}")

    print("[DICT] min/avg/max/mdev")
    dict_mean = statistics.mean(dict_times)
    dict_mdev = sum(abs(dict_mean - time) for time in dict_times) / len(dict_times)
    print(f"[DICT] {min(dict_times):.1f}/{dict_mean:.1f}/{max(dict_times):.1f}/{dict_mdev:.1f}")


if __name__ == "__main__":
    main()
