use clap::Parser;
use std::time::Instant;
// Import logic from our new library (the crate name is the folder name)
// Assuming your folder is named "rust-log-blaster" (dashes become underscores)
use rust_log_blaster::process_logs; 

#[derive(Parser, Debug)]
#[command(version, about = "Batch process log files")]
struct Args {
    #[arg(required = true, num_args(1..))] 
    filenames: Vec<String>,
}

use pyo3::prelude::*; // Import the Python-Rust bridge

// ... (your existing LogRecord struct and process_logs function stay here) ...

// --- THE PYTHON BRIDGE ---

// 1. The Wrapper Function
// This is the function Python will actually call.
// It wraps our pure Rust function 'process_logs'.
#[pyfunction]
fn rust_log_count(filename: String) -> PyResult<u64> {
    // We call our logic. If it fails, we convert the Rust error to a Python error
    match process_logs(&filename) {
        Ok(count) => Ok(count),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
    }
}

// 2. The Module Definition
// This tells Python: "Hi, I am a module named 'rust_log_blaster'. 
// I have one function inside me called 'rust_log_count'."
#[pymodule]
fn rust_log_blaster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_log_count, m)?)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let total_start = Instant::now();

    for filename in &args.filenames {
        println!("-> Processing: {}", filename);
        
        // We call the library function
        match process_logs(filename) {
            Ok(count) => println!("   Found {} ERROR logs.", count),
            Err(e) => eprintln!("   [FAILED] {}: {}", filename, e),
        }
    }

    println!("Total time: {:.2?} seconds", total_start.elapsed());
}