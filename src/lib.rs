use std::{
    arch::x86_64::_rdtsc,
    time::{Duration, Instant},
};

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

pub fn timing_rdtsc<F: FnOnce()>(f: F) -> u64 {
    let start = unsafe { _rdtsc() };
    f();
    let end = unsafe { _rdtsc() };
    end - start
}

pub fn timing_rdtsc_return<T, F: FnOnce() -> T>(f: F) -> (T, u64) {
    let start = unsafe { _rdtsc() };
    let t = f();
    let end = unsafe { _rdtsc() };
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
}
