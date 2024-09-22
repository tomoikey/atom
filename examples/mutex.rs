use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            println!("spawn!");
            s.spawn(|| {
                let mut n = n.lock().unwrap();
                for _ in 0..100 {
                    *n += 1;
                }
                drop(n);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    println!("{}", n.into_inner().unwrap());
}
