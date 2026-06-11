use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let filename = "../data.csv";
    let path = Path::new(filename);
    
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    
    // Skip the header
    lines.next();

    let mut total_sum: f64 = 0.0;
    let mut row_count = 0;

    for line in lines {
        let line = line?;
        
        // Split by comma
        // Using raw string parsing to closely match the C++ std::getline approach
        let mut parts = line.split(',');
        
        // Skip ID
        parts.next();
        
        // Parse value1
        if let Some(v1_str) = parts.next() {
            let v1: f64 = v1_str.parse().unwrap_or(0.0);
            
            // Parse value2
            if let Some(v2_str) = parts.next() {
                let v2: f64 = v2_str.parse().unwrap_or(0.0);
                
                total_sum += v1 * v2;
                row_count += 1;
            }
        }
    }

    println!("Rust Data Processing Complete");
    println!("Rows processed: {}", row_count);
    println!("Total Sum of Products: {:.4}", total_sum);

    Ok(())
}
