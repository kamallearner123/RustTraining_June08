mod html_generator;
mod parser;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ValgrindReportAnalyzer")]
#[command(author = "System Architect")]
#[command(version = "1.0")]
#[command(about = "Parses Valgrind text logs and generates beautiful HTML reports.", long_about = None)]
struct Args {
    /// The path to the Valgrind log file (e.g., report.log)
    input: PathBuf,

    /// The path for the generated HTML report
    #[arg(short, long, default_value = "report.html")]
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("[*] Reading Valgrind log from: {}", args.input.display());

    match parser::parse_log(&args.input) {
        Ok(report) => {
            println!("[+] Parsed report for PID {} running '{}'", report.pid, report.command);
            println!("    - Found {} error(s)", report.errors.len());
            
            match html_generator::generate_html(&report, &args.output) {
                Ok(_) => {
                    println!("[+] Successfully generated HTML report at: {}", args.output);
                }
                Err(e) => {
                    eprintln!("[-] Failed to write HTML file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("[-] Failed to parse log file: {}", e);
        }
    }
}
