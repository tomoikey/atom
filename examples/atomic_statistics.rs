use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    let num_done = AtomicUsize::new(0);
    let total_time = AtomicU64::new(0);
    let max_time = AtomicU64::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    let start = std::time::Instant::now();
                    for _ in 0..100000000 {
                        // 何かの処理
                    }
                    let elapsed = start.elapsed().as_millis() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(elapsed, Relaxed);
                    max_time.fetch_max(elapsed, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            let total_time = total_time.load(Relaxed);
            let average_time = if n != 0 { total_time / n as u64 } else { 0 };
            let max = max_time.load(Relaxed);
            println!(
                "Working.. {}/100 done, total time: {}ms, max time: {}ms, average time: {}ms",
                n, total_time, max, average_time
            );
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
