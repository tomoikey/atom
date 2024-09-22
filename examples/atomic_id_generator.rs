use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(move || {
                for _ in 0..10000 {
                    let id = generate_id();
                    println!("Generated ID: {}", id);
                }
            });
        }
    })
}

fn generate_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_ID.fetch_add(1, Relaxed)
}
