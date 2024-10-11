use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
fn main() {
    for _ in 0..100000 {
        f();
    }
}

fn f() {
    let a = thread::spawn(|| {
        a();
    });
    let b = thread::spawn(|| {
        b();
    });
    a.join().unwrap();
    b.join().unwrap();
    X.store(0, Relaxed);
}

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}
fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
    assert!(a <= b && b <= c && c <= d);
}