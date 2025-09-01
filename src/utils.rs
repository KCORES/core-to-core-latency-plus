use std::time::Duration;
use quanta::Clock;
use crate::bench::Count;

pub fn black_box<T>(dummy: T) -> T {
    unsafe { std::ptr::read_volatile(&dummy) }
}

pub fn delay_cycles(num_iterations: usize) {
    static VALUE: usize = 0;
    for _ in 0..num_iterations {
        // unsafe { std::arch::asm!("nop"); } might not work on all platforms
        black_box(&VALUE);
    }
}

// Returns the duration of doing `num_iterations` of clock.raw()
pub fn clock_read_overhead_sum(clock: &Clock, num_iterations: Count) -> Duration {
    let start = clock.raw();
    for _ in 0..(num_iterations-1) {
        black_box(clock.raw());
    }
    let end = clock.raw();
    clock.delta(start, end)
}

// This big feature condition is on CpuId::default(), we'll use the same.
#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
pub fn get_cpuid() -> Option<raw_cpuid::CpuId> {
    Some(raw_cpuid::CpuId::default())
}

#[cfg(not(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
)))]
pub fn get_cpuid() -> Option<raw_cpuid::CpuId> {
    None
}

pub fn assert_rdtsc_usable(clock: &quanta::Clock) {
    let cpuid = get_cpuid().expect("This benchmark is only compatible with x86");

    assert!(cpuid.get_advanced_power_mgmt_info().expect("CPUID failed").has_invariant_tsc(),
        "This benchmark only runs with a TscInvariant=true");

    const NUM_ITERS: Count = 10_000;
    let clock_read_overhead = clock_read_overhead_sum(&clock, NUM_ITERS).as_nanos() as f64 / NUM_ITERS as f64;
    eprintln!("Reading the clock via RDTSC takes {:.2}ns", clock_read_overhead);
    assert!((0.1..1000.0).contains(&clock_read_overhead), "The timing to read the clock is either not-consistant or too slow");
}

fn get_cpu_brand_from_proc() -> Option<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("/proc/cpuinfo").ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let line_lower = line.to_lowercase();
        if line_lower.starts_with("model name") || line_lower.starts_with("hardware") {
            return line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        }
    }
    None
}

pub fn get_cpu_brand() -> Option<String> {
    // only support intel and amd
    if let Some(cpuid) = get_cpuid() {
        if let Some(brand) = cpuid.get_processor_brand_string() {
            return Some(brand.as_str().to_string());
        }
    }
    // fallback to reading /proc/cpuinfo
    get_cpu_brand_from_proc()
}

pub fn show_cpuid_info() {
    if let Some(brand) = get_cpu_brand() {
        eprintln!("CPU: {}", brand);
    }
}
