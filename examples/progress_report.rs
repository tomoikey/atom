use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);
    thread::scope(|s| {
        // 1 つのバックグラウンドスレッドで 100 個のアイテムをすべて処理する
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_millis(500));
                num_done.store(i + 1, Relaxed);
            }
        });
        // メインスレッドは、毎秒 1 回状態を更新する
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
