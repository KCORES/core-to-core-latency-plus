mod bench;
mod utils;

use bench::Count;
use std::sync::Arc;
use clap::Parser;
use quanta::Clock;
use crate::bench::run_bench;
use serde_json::json;
use reqwest::blocking::Client;


const DEFAULT_NUM_SAMPLES: Count = 300;
const DEFAULT_NUM_ITERATIONS_PER_SAMPLE: Count = 1000;

#[derive(Clone)]
#[derive(clap::Parser)]
pub struct CliArgs {
    /// The number of iterations per sample
    #[clap(default_value_t = DEFAULT_NUM_ITERATIONS_PER_SAMPLE, value_parser)]
    num_iterations: Count,

    /// The number of samples
    #[clap(default_value_t = DEFAULT_NUM_SAMPLES, value_parser)]
    num_samples: Count,

    /// Outputs the mean latencies in CSV format on stdout
    #[clap(long, value_parser)]
    csv: bool,

    /// Select which benchmark to run, in a comma delimited list, e.g., '1,3' {n}
    /// 1: CAS latency on a single shared cache line. {n}
    /// 2: Single-writer single-reader latency on two shared cache lines. {n}
    /// 3: One writer and one reader on many cache line, using the clock. {n}
    #[clap(short, long, default_value="1", require_delimiter=true, value_delimiter=',', value_parser)]
    bench: Vec<usize>,

    /// Specify the cores by id that should be used, comma delimited. By default all cores are used.
    #[clap(short, long, require_delimiter=true, value_delimiter=',', value_parser)]
    cores: Vec<usize>,

    /// Upload benchmark results
    #[clap(long)]
    upload: bool,
}

fn main() {
    let args = CliArgs::parse();

    let cores = core_affinity::get_core_ids().expect("get_core_ids() failed");

    let cores = if !args.cores.is_empty() {
        args.cores.iter().copied()
            .map(|cid| *cores.iter().find(|c| c.id == cid)
                .unwrap_or_else(||panic!("Core {} not found. Available: {:?}", cid, &cores)))
            .collect()
    } else {
        cores
    };

    utils::show_cpuid_info();
    eprintln!("Num cores: {}", cores.len());
    eprintln!("Num iterations per samples: {}", args.num_iterations);
    eprintln!("Num samples: {}", args.num_samples);
    #[cfg(target_os = "macos")]
    eprintln!("{}", ansi_term::Color::Red.bold().paint("WARN macOS may ignore thread-CPU affinity (we can't select a CPU to run on). Results may be inaccurate"));

    let clock = Arc::new(Clock::new());

    let mut all_results = Vec::new();

    for b in &args.bench {
        let result = match b {
            1 => {
                eprintln!();
                eprintln!("1) CAS latency on a single shared cache line");
                eprintln!();
                run_bench(&cores, &clock, &args, bench::cas::Bench::new())
            }
            2 => {
                eprintln!();
                eprintln!("2) Single-writer single-reader latency on two shared cache lines");
                eprintln!();
                run_bench(&cores, &clock, &args, bench::read_write::Bench::new())
            }
            3 => {
                utils::assert_rdtsc_usable(&clock);
                eprintln!();
                eprintln!("3) Message passing. One writer and one reader on many cache line");
                eprintln!();
                run_bench(&cores, &clock, &args, bench::msg_passing::Bench::new(args.num_iterations))
            }
            _ => panic!("--bench should be 1, 2 or 3"),
        };
        if let Some(csv_data) = result {
            all_results.push(csv_data);
        }
    }

    if args.upload {
        upload_results(all_results);
    }
}

fn upload_results(results: Vec<String>) {
    let cpu_name = utils::get_cpu_brand().unwrap_or_else(|| "Unknown".to_string());
    
    println!("Please enter your name (or just press Enter for 'anonymous'):");
    let mut commit_by = String::new();
    std::io::stdin().read_line(&mut commit_by).expect("Failed to read input");
    commit_by = commit_by.trim().to_string();
    if commit_by.is_empty() {
        commit_by = "anonymous".to_string();
    }

    let csv_data = results.join("\n");

    let payload = json!({
        "cpu_name": cpu_name,
        "commit_by": commit_by,
        "file": csv_data
    });

    let client = Client::new();
    match client.post("https://core-to-core-latency.kcores.com/api/v1/commit")
        .json(&payload)
        .send() {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(json) = response.json::<serde_json::Value>() {
                    if let Some(id) = json["id"].as_u64() {
                        println!("Upload successful!");
                        println!("You can view the results at the following URL:");
                        println!("https://core-to-core-latency.kcores.com/results/{}", id);
                    } else {
                        println!("Upload successful, but unable to get result ID.");
                    }
                } else {
                    println!("Upload successful, but unable to parse response.");
                }
            } else {
                println!("Upload failed: HTTP status code {}", response.status());
            }
        },
        Err(e) => println!("Upload failed: {}", e),
    }
}
