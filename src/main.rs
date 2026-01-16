use std::error::Error;
use std::fs::File;
use std::time::Instant;
use serde::Deserialize;
use clap::Parser;

// --- ARGUMENT DEFINITION ---
#[derive(Parser, Debug)]
#[command(version, about = "Batch process log files")]
struct Args {
    /// List of log files to process (space separated)
    // We changed this from String to Vec<String> to accept a list
    #[arg(required = true, num_args(1..))] 
    filenames: Vec<String>,
}

// --- DATA DEFINITION ---
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LogRecord {
    timestamp: String,
    level: String,
    message: String,
    response_time_ms: u32,
}

fn main() {
    let args = Args::parse();

    println!("--- RUST LOG BLASTER (BATCH MODE) ---");
    
    // We start a "Global" timer
    let total_start = Instant::now();

    // --- THE BATCH LOOP ---
    // We iterate through every filename the user provided
    for filename in &args.filenames {
        println!("\n-> Processing: {}", filename);
        
        // We call our logic function for THIS specific file
        // If it fails, we print the error but KEEP GOING to the next file.
        if let Err(e) = process_logs(filename) {
            eprintln!("   [FAILED] Could not process {}: {}", filename, e);
        }
    }

    let total_duration = total_start.elapsed();
    println!("\n--- DONE ---");
    println!("Total time for all files: {:.2?} seconds", total_duration);
}

fn process_logs(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut error_count = 0;

    for result in rdr.deserialize() {
        let record: LogRecord = result?;
        if record.level == "ERROR" {
            error_count += 1;
        }
    }

    println!("   Found {} ERROR logs.", error_count);
    Ok(())
}