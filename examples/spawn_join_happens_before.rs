use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    for _ in 0..10000 {
        f();
    }
}

fn f() {
    X.store(1, Relaxed);
    let t = thread::spawn(g);
    X.store(2, Relaxed);
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn g() {
    let x = X.load(Relaxed);
    println!("{x}");
    assert!(x == 1 || x == 2);
}