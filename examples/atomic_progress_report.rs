use std::thread;

fn main() {
    let num_done = &std::sync::atomic::AtomicUsize::new(0);
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for _ in 0..25 {
                    println!("Thread {} working...", t);
                    thread::sleep(std::time::Duration::from_millis(100));
                    num_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {}/100 done", n);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
