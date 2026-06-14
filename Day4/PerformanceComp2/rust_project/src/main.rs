use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let filename = "words.txt";
    let path = Path::new(filename);
    
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut word_counts: HashMap<String, i32> = HashMap::new();
    let mut total_words = 0;

    // Read line by line
    for line in reader.lines() {
        let line = line?;
        // For whitespace separated words in a line (if any, our generator puts one word per line)
        for word in line.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
            total_words += 1;
        }
    }

    // Convert to vector for sorting
    let mut sorted_counts: Vec<(&String, &i32)> = word_counts.iter().collect();
    
    // Sort descending by frequency
    sorted_counts.sort_unstable_by(|a, b| b.1.cmp(a.1));

    println!("Rust Hash Map Processing Complete");
    println!("Total Words Processed: {}", total_words);
    println!("Unique Words Found: {}", word_counts.len());
    
    println!("Top 5 Words:");
    for i in 0..std::cmp::min(5, sorted_counts.len()) {
        println!("  {}: {}", sorted_counts[i].0, sorted_counts[i].1);
    }

    Ok(())
}
