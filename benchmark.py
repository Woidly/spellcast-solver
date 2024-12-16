import json
import random
import string
import statistics
import subprocess
from collections import defaultdict
from argparse import ArgumentParser
from tqdm import tqdm


def generate_board():
    return ''.join(random.choices(string.ascii_lowercase, k=25))


def run_solver(swaps, board, solver_threads):
    cmd = [
        "./target/release/spellcast-solver",
        "-t", str(solver_threads),
        "-s", str(swaps),
        "-b", board,
        "-f", "json",
    ]
    sub = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(sub.stdout)['elapsed_ms']


def main():
    # Parse command-line arguments
    parser = ArgumentParser(description="Benchmark the spellcast solver.")
    parser.add_argument(
        "-b", "--boards", 
        type=int, 
        default=100, 
        help="number of boards to generate and solve (def=100)"
    )
    parser.add_argument(
        "-t", "--threads", 
        type=int, 
        default=1, 
        help="number of solver threads (def=1)"
    )
    args = parser.parse_args()

    board_count = args.boards
    solver_threads = args.threads

    print(f"Running benchmark for {board_count} boards")
    times = defaultdict(list)
    dict_times = []

    for i in tqdm(range(board_count)):
        i_swaps = i % 4
        time = run_solver(i_swaps, generate_board(), solver_threads)
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
