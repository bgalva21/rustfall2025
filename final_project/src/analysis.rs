use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

#[derive(Debug, Default)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessingError {
    pub filename: String,
    pub operation: String,
    pub message: String,
}

#[derive(Clone)]
pub struct CancellationToken {
    flag: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            flag: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }
    pub fn cancel(&self) {
        if let Ok(mut f) = self.flag.lock() {
            *f = true;
        }
    }
    pub fn is_cancelled(&self) -> bool {
        self.flag.lock().map(|v| *v).unwrap_or(true)
    }
}

//collection of all txt files
pub fn collect_txt_files(dirs: &[PathBuf]) -> (Vec<PathBuf>, Vec<ProcessingError>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();

    for dir in dirs {
        collect_one_dir(dir, &mut files, &mut errors);
    }

    (files, errors)
}

fn collect_one_dir(dir: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<ProcessingError>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            errors.push(ProcessingError {
                filename: dir.display().to_string(),
                operation: "read_dir".to_string(),
                message: e.to_string(),
            });
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                errors.push(ProcessingError {
                    filename: dir.display().to_string(),
                    operation: "read_dir_entry".to_string(),
                    message: e.to_string(),
                });
                continue;
            }
        };

        let path = entry.path();
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                errors.push(ProcessingError {
                    filename: path.display().to_string(),
                    operation: "metadata".to_string(),
                    message: e.to_string(),
                });
                continue;
            }
        };

        if meta.is_dir() {
            collect_one_dir(&path, files, errors);
        } else if meta.is_file() {
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("txt")) == Some(true) {
                files.push(path);
            }
        }
    }
}

pub fn analyze_file(path: &Path, cancel: &CancellationToken) -> FileAnalysis {
    let start = Instant::now();
    let filename = path.display().to_string();

    let mut errors: Vec<ProcessingError> = Vec::new();
    let mut stats = FileStats::default();

    if cancel.is_cancelled() {
        errors.push(ProcessingError {
            filename: filename.clone(),
            operation: "cancel".to_string(),
            message: "Cancelled before start".to_string(),
        });
        return FileAnalysis {
            filename,
            stats,
            errors,
            processing_time: start.elapsed(),
        };
    }


    match fs::metadata(path) {
        Ok(meta) => stats.size_bytes = meta.len(),
        Err(e) => errors.push(err(&filename, "metadata", e)),
    }

    // opening file
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            errors.push(err(&filename, "open", e));
            return FileAnalysis {
                filename,
                stats,
                errors,
                processing_time: start.elapsed(),
            };
        }
    };

    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    loop {
        if cancel.is_cancelled() {
            errors.push(ProcessingError {
                filename: filename.clone(),
                operation: "cancel".to_string(),
                message: "Cancelled during read".to_string(),
            });
            break;
        }

        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                stats.line_count += 1;
                stats.word_count += line.split_whitespace().count();
                for ch in line.chars() {
                    *stats.char_frequencies.entry(ch).or_insert(0) += 1;
                }
            }
            Err(e) => {
                errors.push(err(&filename, "read_line", e));
                break;
            }
        }
    }

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start.elapsed(),
    }
}

fn err(filename: &str, operation: &str, e: io::Error) -> ProcessingError {
    ProcessingError {
        filename: filename.to_string(),
        operation: operation.to_string(),
        message: e.to_string(),
    }
}
