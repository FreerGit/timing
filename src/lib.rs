use std::arch::asm;
use std::time::{Duration, Instant};

pub fn timing<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

pub fn timing_return<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let t = f();
    (t, start.elapsed())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn rdtsc_start() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "mfence",
            "lfence",
            "rdtsc",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn rdtsc_stop() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "rdtsc",
            "lfence",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

#[cfg(target_arch = "x86_64")]
pub fn timing_rdtsc<F: FnOnce()>(f: F) -> u64 {
    let start = rdtsc_start();
    f();
    let end = rdtsc_stop();
    end - start
}

#[cfg(target_arch = "x86_64")]
pub fn timing_rdtsc_return<T, F: FnOnce() -> T>(f: F) -> (T, u64) {
    let start = rdtsc_start();
    let t = f();
    let end = rdtsc_stop();
    (t, end - start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_timing() {
        let time = timing(|| {
            println!("A fn that takes a few micros");
        });

        assert!(!time.is_zero());
    }

    #[test]
    fn simple_timing_with_return() {
        let (val, time) = timing_return(|| {
            println!("A fn that takes a few micros");
            5
        });

        assert!(!time.is_zero());
        assert!(val == 5);
    }

    #[test]
    fn rdtsc_timing() {
        let counter = timing_rdtsc(|| {
            println!("...");
        });

        assert!(counter != 0);
        println!("{counter}");
    }

    #[test]
    fn rdtsc_timing_return() {
        let (val, counter) = timing_rdtsc_return(|| {
            println!("...");
            42
        });

        assert!(counter != 0);
        assert!(val == 42);
    }
}
