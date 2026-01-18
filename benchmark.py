import time
import rust_log_blaster  # This is YOUR Rust library!
import csv

FILENAME = "large_log.csv"

def python_process_logs(filepath):
    """The standard slow Python way"""
    count = 0
    with open(filepath, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            if row['level'] == 'ERROR':
                count += 1
    return count

print(f"--- BENCHMARKING: {FILENAME} ---")

# 1. Test Pure Python
start = time.time()
print("Running Pure Python...", end="", flush=True)
py_count = python_process_logs(FILENAME)
py_time = time.time() - start
print(f" DONE! ({py_time:.4f}s)")

# 2. Test Rust Implementation
start = time.time()
print("Running Rust Module...", end="", flush=True)
# This looks like Python, but it runs at C++ speeds
rust_count = rust_log_blaster.rust_log_count(FILENAME)
rust_time = time.time() - start
print(f" DONE! ({rust_time:.4f}s)")

# 3. Report
print("\n--- RESULTS ---")
print(f"Python Time: {py_time:.4f}s")
print(f"Rust Time:   {rust_time:.4f}s")
print(f"Speedup:     {py_time / rust_time:.1f}x FASTER 🚀")
print(f"Counts Match: {py_count == rust_count}")