import csv
import time

def process_logs_python(filename="large_log.csv"):
    print("--- PYTHON LOG BLASTER ---")
    start_time = time.time()
    
    error_count = 0
    
    with open(filename, "r", encoding="utf-8") as f:
        reader = csv.reader(f)
        header = next(reader) # Skip header
        
        for row in reader:
            # Python has to figure out what 'row' is every single time (Dynamic Typing)
            if row[1] == "ERROR":
                error_count += 1
                
    end_time = time.time()
    print(f"Found {error_count} ERROR logs.")
    print(f"Time taken: {end_time - start_time:.2f} seconds")

if __name__ == "__main__":
    process_logs_python()