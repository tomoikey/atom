use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    let x = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..5 {
            s.spawn(|| {
                for _ in 0..10 {
                    increment(&x);
                }
            });
        }
    });
    println!("X: {}", x.load(Relaxed));

    for _ in 0..5 {
        println!("ID: {}", generate_id());
    }

    for _ in 0..5 {
        println!("ID: {}", generate_id_fetch_update());
    }
    
    for _ in 0..5 {
        println!("KEY: {}", get_key());
    }
}

fn increment(x: &AtomicU64) {
    let mut current = x.load(Relaxed);
    loop {
        let new = current + 1;
        match x.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => break,
            Err(y) => current = y,
        }
    }
}

#[allow(dead_code)]
fn increment_fetch_update(x: &AtomicU64) {
    x.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1)).expect("ID 多すぎるンゴ");
}

fn generate_id() -> u64 {
    static ID: AtomicU64 = AtomicU64::new(0);
    let mut current = ID.load(Relaxed);
    loop {
        assert!(current < 1000, "ID 多すぎるンゴ");
        let new = current + 1;
        match ID.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(old) => return old,
            Err(c) => current = c,
        }
    }
}

fn generate_id_fetch_update() -> u64 {
    static ID: AtomicU64 = AtomicU64::new(0);
    ID.fetch_update(Relaxed, Relaxed, |n| {
        if n < 2000 {
            Some(n + 1)
        } else {
            None
        }
    }).expect("ID 多すぎるンゴ")
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let generated = 100;
        match KEY.compare_exchange(0, generated, Relaxed, Relaxed) {
            Ok(_) => generated,
            Err(k) => k,
        }
    } else {
        key
    }
}
