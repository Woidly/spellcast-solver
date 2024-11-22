from collections import defaultdict
import subprocess
import random
import string
import statistics

from tqdm import tqdm

# Config
BOARD_COUNT = 100
SOLVER_THREADS = 1


def generate_board():
    return ''.join(random.choices(string.ascii_lowercase, k=25))


def run_solver(swaps, board):
    cmd = ["./target/release/spellcast-solver", "-t", str(SOLVER_THREADS), "solver", "-s", str(swaps), "-b", board]
    result = subprocess.run(cmd, capture_output=True, text=True)
    for line in result.stdout.splitlines():
        if "Found" in line:
            return float(line.split()[-1][:-2])
    raise Exception("failed to parse stdout")


if __name__ == "__main__":
    print(f"Running benchmark for {BOARD_COUNT} boards")
    times = defaultdict(list)
    for i in tqdm(range(BOARD_COUNT)):
        swaps = i % 4
        times[swaps].append(run_solver(swaps, generate_board()))
    print("Benchmark done")
    print("[X swaps] min/avg/max/mdev")
    for swaps in range(4):
        if times[swaps]:
            timex = times[swaps]
            mean = statistics.mean(timex)
            mdev = sum(abs(mean - time) for time in timex) / len(timex)
            print(f"[{swaps} swaps] {min(timex):.1f}/{mean:.1f}/{max(timex):.1f}/{mdev:.1f}")


