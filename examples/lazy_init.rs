use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0); // 計算前であることを 0 で表現できると仮定する
    let mut x = X.load(Relaxed);
    if x == 0 {
        // 複数のスレッドが同時にアクセスすると、競合が発生する (データ競合ではない)
        x += 1;
        X.store(x, Relaxed);
    }
    x
}

fn main() {
    for _ in 0..5 {
        println!("X: {}", get_x());
        sleep(std::time::Duration::from_secs(1));
    }
}