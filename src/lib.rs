use std::error::Error;
use std::fs::File;
use serde::Deserialize;
// --- ADDED THIS LINE ---
use pyo3::prelude::*; 

// 1. The Struct
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LogRecord {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub response_time_ms: u32,
}

// 2. The Logic Function
pub fn process_logs(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut error_count = 0;

    for result in rdr.deserialize() {
        if let Ok(record) = result {
            let record: LogRecord = record;
            if record.level == "ERROR" {
                error_count += 1;
            }
        }
    }

    Ok(error_count)
}

// --- THE MISSING BRIDGE (YOU MUST HAVE THIS) ---

// 3. The Wrapper Function
#[pyfunction]
fn rust_log_count(filename: String) -> PyResult<u64> {
    match process_logs(&filename) {
        Ok(count) => Ok(count),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string())),
    }
}

// 4. The Module Definition
// The name of this function "rust_log_blaster" MUST match Cargo.toml [lib] name
#[pymodule]
fn rust_log_blaster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_log_count, m)?)?;
    Ok(())
}