use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(move || {
                for _ in 0..25 {
                    thread::sleep(std::time::Duration::from_millis(100));
                    increment(num_done);
                }
            });
        }
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {}/100 done", n);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn increment(a: &AtomicUsize) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange_weak(current, new, Relaxed, Relaxed) {
            Ok(_) => break,
            Err(x) => {
                println!("Failed to increment, retrying");
                current = x
            }
        }
    }
}
