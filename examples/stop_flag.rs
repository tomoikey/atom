use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // 何か仕事をするためのスレッドを起動
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            thread::sleep(std::time::Duration::from_secs(5));
            println!("working...");
        }
    });
    // メインスレッドを使ってユーザ入力を受け付ける
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    // バックグラウンドスレッドに停止するように知らせる
    STOP.store(true, Relaxed);
    // バックグラウンドスレッドが終了するまで待つ
    background_thread.join().unwrap();
}
