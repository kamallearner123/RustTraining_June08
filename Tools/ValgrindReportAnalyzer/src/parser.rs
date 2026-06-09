use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
pub struct ValgrindReport {
    pub pid: String,
    pub command: String,
    pub errors: Vec<ValgrindError>,
    pub heap_summary: Vec<String>,
    pub leak_summary: Vec<String>,
    pub error_summary: String,
}

#[derive(Debug)]
pub struct ValgrindError {
    pub kind: String,
    pub backtrace: Vec<String>,
}

#[derive(PartialEq)]
enum ParserState {
    General,
    HeapSummary,
    LeakSummary,
}

pub fn parse_log<P: AsRef<Path>>(path: P) -> io::Result<ValgrindReport> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut report = ValgrindReport::default();
    let mut current_error: Option<ValgrindError> = None;
    let mut state = ParserState::General;

    // Matches `==PID== ` or `==PID==`
    let prefix_re = Regex::new(r"^==(\d+)==\s?(.*)").unwrap();

    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = prefix_re.captures(&line) {
            let pid = captures.get(1).map_or("", |m| m.as_str());
            let content = captures.get(2).map_or("", |m| m.as_str()).trim();

            if report.pid.is_empty() && !pid.is_empty() {
                report.pid = pid.to_string();
            }

            if content.is_empty() {
                // Empty valgrind line often separates errors.
                if let Some(err) = current_error.take() {
                    if !err.kind.contains("HEAP SUMMARY") && !err.kind.contains("LEAK SUMMARY") && !err.kind.contains("ERROR SUMMARY") {
                        report.errors.push(err);
                    }
                }
                state = ParserState::General;
                continue;
            }

            if content.starts_with("Command:") {
                report.command = content.replace("Command:", "").trim().to_string();
                continue;
            }

            if content.starts_with("HEAP SUMMARY:") {
                state = ParserState::HeapSummary;
                continue;
            }

            if content.starts_with("LEAK SUMMARY:") {
                state = ParserState::LeakSummary;
                continue;
            }

            if content.starts_with("ERROR SUMMARY:") {
                report.error_summary = content.to_string();
                continue;
            }

            match state {
                ParserState::HeapSummary => {
                    report.heap_summary.push(content.to_string());
                }
                ParserState::LeakSummary => {
                    report.leak_summary.push(content.to_string());
                }
                ParserState::General => {
                    // Check if it looks like a backtrace line (starts with "at" or "by")
                    if content.starts_with("at 0x") || content.starts_with("by 0x") || content.starts_with("Address 0x") {
                        if let Some(ref mut err) = current_error {
                            err.backtrace.push(content.to_string());
                        }
                    } else if !content.starts_with("Memcheck") && !content.starts_with("Copyright") && !content.starts_with("Using Valgrind") {
                        // Start of a new error or log message
                        if let Some(err) = current_error.take() {
                            report.errors.push(err);
                        }
                        current_error = Some(ValgrindError {
                            kind: content.to_string(),
                            backtrace: Vec::new(),
                        });
                    }
                }
            }
        } else {
            // Not a valgrind line, ignore or add to general logs if needed.
        }
    }

    // Push the last error if file ended without an empty line
    if let Some(err) = current_error {
        if !err.kind.contains("HEAP SUMMARY") && !err.kind.contains("LEAK SUMMARY") && !err.kind.contains("ERROR SUMMARY") {
            report.errors.push(err);
        }
    }

    Ok(report)
}
