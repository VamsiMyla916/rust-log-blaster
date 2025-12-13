import csv
import time
import random

# Generates a 500MB+ file (approx 10 million rows)
def generate_logs(filename="large_log.csv", rows=10_000_000):
    print(f"Generating {rows} rows... (This will take about 10-20 seconds)")
    start_time = time.time()
    
    levels = ["INFO", "DEBUG", "WARN", "ERROR"]
    msgs = ["User logged in", "Failed to connect", "Timeout", "Data sync"]
    
    with open(filename, "w", newline='') as f:
        writer = csv.writer(f)
        writer.writerow(["timestamp", "level", "message", "response_time_ms"])
        
        for i in range(rows):
            # Write a row
            writer.writerow([
                f"2025-12-13 10:{random.randint(10,59)}:{random.randint(10,59)}",
                random.choice(levels),
                random.choice(msgs),
                random.randint(10, 5000)
            ])
            if i % 1_000_000 == 0:
                print(f"{i} rows written...")

    print(f"Done! File generated in {time.time() - start_time:.2f} seconds.")

if __name__ == "__main__":
    generate_logs()