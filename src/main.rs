use std::error::Error;
use std::fs::File;
use std::time::Instant;
use serde::Deserialize;

// 1. DEFINING THE SHAPE OF DATA
// We tell Rust what a "Log Record" looks like.
// Derive(Deserialize) is magic that lets the 'csv' library fill this struct automatically.
#[derive(Debug, Deserialize)]
struct LogRecord {
    timestamp: String,
    level: String,
    message: String,
    response_time_ms: u32,
}

fn main() {
    println!("--- RUST LOG BLASTER ---");

    // Start the timer
    let start = Instant::now();

    // 2. ERROR HANDLING
    // We call the function. If it returns an error (Err), we print it.
    // loops and logic are inside 'process_logs'.
    if let Err(e) = process_logs("large_log.csv") {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    // Stop the timer
    let duration = start.elapsed();
    println!("Time taken: {:.2?} seconds", duration);
}

// 3. THE LOGIC FUNCTION
// Returns a Result type: Ok(()) if good, Box<dyn Error> if bad.
fn process_logs(file_path: &str) -> Result<(), Box<dyn Error>> {
    
    // TEACHING MOMENT: OWNERSHIP
    // We are passing 'file_path' (a string) to File::open.
    // We are NOT giving ownership of the file to Rust yet, just opening a handle to it.
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    let mut error_count = 0;

    // 4. THE LOOP (Streaming Data)
    // We iterate over the file one row at a time.
    // This is memory efficient. We don't load 500MB into RAM.
    for result in rdr.deserialize() {
        
        // TEACHING MOMENT: BORROWING vs MOVING
        // 'record' is a new variable that OWNS the data for this specific row.
        // If we didn't use it, Rust would clean it up (drop it) immediately after this bracket.
        let record: LogRecord = result?;

        // Check if the level is "ERROR"
        // We are 'borrowing' the level string here using & to look at it.
        if record.level == "ERROR" {
            error_count += 1;
        }
        
        // End of loop: 'record' is dropped (deleted) from memory here.
        // This is why Rust needs no Garbage Collector!
    }

    println!("Found {} ERROR logs.", error_count);
    Ok(())
}