mod thread_pool;
mod analysis;

use analysis::{CancellationToken, FileAnalysis};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use thread_pool::ThreadPool;

#[derive(Debug, Clone, Copy)]
enum Status {
    Queued,
    Processing,
    Done,
    Failed,
    Cancelled,
}

#[derive(Debug)]
struct ProgressState {
    total: usize,
    done: usize,
    per_file: HashMap<String, Status>,
    error_count: usize,
    started_at: Instant,
    times: Vec<Duration>,
}

fn print_file_report(a: &FileAnalysis) {
    let display_name = std::path::Path::new(&a.filename)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&a.filename);

    println!("\n==============================");
    println!("Processing {}", display_name);

    println!("Word count: {}", a.stats.word_count);
    println!("Line count: {}", a.stats.line_count);
    println!("Size (bytes): {}", a.stats.size_bytes);
    println!("Processing time: {:?}", a.processing_time);

    if a.errors.is_empty() {
        println!("Errors: none");
    } else {
        println!("Errors ({}):", a.errors.len());
        for e in &a.errors {
            println!("  - {} | {}: {}", e.filename, e.operation, e.message);
        }
    }
    println!("==============================\n");
}

fn main() {
    //can accept multiple dirs from cli, defaults to data dir
    let dirs: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    let dirs = if dirs.is_empty() {
        vec![PathBuf::from("data")]
    } else {
        dirs
    };

    // get all .txt files from dirs
    let (files, dir_errors) = analysis::collect_txt_files(&dirs);
    println!("Found {} .txt files", files.len());
    for e in &dir_errors {
        eprintln!("[DIR ERROR] {} {}: {}", e.filename, e.operation, e.message);
    }

    if files.is_empty() {
        println!("No files to process.");
        return;
    }

    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let pool = ThreadPool::new(threads);

    let cancel = CancellationToken::new();

    // shared progress
    let progress = Arc::new(Mutex::new(ProgressState {
        total: files.len(),
        done: 0,
        per_file: HashMap::new(),
        error_count: 0,
        started_at: Instant::now(),
        times: Vec::new(),
    }));

    //per file status
    {
        let mut p = progress.lock().unwrap();
        for f in &files {
            p.per_file.insert(f.display().to_string(), Status::Queued);
        }
    }

    // reporter thread
    let progress_for_reporter = Arc::clone(&progress);
    let cancel_for_reporter = cancel.clone();
    let reporter = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(300));
        let p = progress_for_reporter.lock().unwrap();
        let elapsed = p.started_at.elapsed();
        eprintln!(
            "[PROGRESS] {}/{} done | errors={} | elapsed={:?} | cancelled={}",
            p.done,
            p.total,
            p.error_count,
            elapsed,
            cancel_for_reporter.is_cancelled()
        );
        if p.done >= p.total {
            break;
        }
    });

    // Enter to cancel
    let cancel_for_stdin = cancel.clone();
    std::thread::spawn(move || {
        let mut s = String::new();
        let _ = std::io::stdin().read_line(&mut s);
        cancel_for_stdin.cancel();
        eprintln!("[CANCEL] Cancellation requested.");
    });

    // using channel to collect results
    let (result_tx, result_rx) = mpsc::channel::<FileAnalysis>();

    // submit job
    for path in files {
        let tx = result_tx.clone();
        let progress = Arc::clone(&progress);
        let cancel = cancel.clone();

        pool.execute(move || {
            let filename = path.display().to_string();

            // mark as proccessing
            {
                let mut p = progress.lock().unwrap();
                p.per_file.insert(filename.clone(), Status::Processing);
            }


            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                println!("Processing {}...", name);
            } else {
                println!("Processing {}...", filename);
            }

            
            let analysis = analysis::analyze_file(&path, &cancel);

            // update
            {
                let mut p = progress.lock().unwrap();

                let status = if cancel.is_cancelled() {
                    Status::Cancelled
                } else if analysis.errors.is_empty() {
                    Status::Done
                } else {
                    Status::Failed
                };

                p.per_file.insert(filename, status);
                p.done += 1;

                if !analysis.errors.is_empty() {
                    p.error_count += analysis.errors.len();
                }

                p.times.push(analysis.processing_time);
            }

            let _ = tx.send(analysis);
        });
    }

    drop(result_tx);

    // collect results
    let mut results: Vec<FileAnalysis> = Vec::new();
    for r in result_rx {
        results.push(r);
    }

    // wainting for reporter
    let _ = reporter.join();

    //sort by filename
    results.sort_by(|a, b| a.filename.cmp(&b.filename));


    //file stats
    for a in &results {
        print_file_report(a);
    }

    // timing stats
    let (min_t, max_t, avg_t) = {
        let p = progress.lock().unwrap();
        if p.times.is_empty() {
            (Duration::ZERO, Duration::ZERO, Duration::ZERO)
        } else {
            let mut min = p.times[0];
            let mut max = p.times[0];
            let mut total = Duration::ZERO;
            for t in &p.times {
                if *t < min {
                    min = *t;
                }
                if *t > max {
                    max = *t;
                }
                total += *t;
            }
            (min, max, total / (p.times.len() as u32))
        }
    };

    println!("Processed {} files", results.len());
    println!("Timing: min={:?} max={:?} avg={:?}", min_t, max_t, avg_t);

    
    println!("Directories processed:");
    for d in dirs {
        println!("  - {}", d.display());
    }

    println!("All files processed successfully.");
}
