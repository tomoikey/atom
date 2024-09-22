use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let n: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let maybe_number = n.lock().unwrap().pop_back();
            if let Some(number) = maybe_number {
                dbg!(number);
            } else {
                thread::park();
            }
        });

        for i in 0.. {
            n.lock().unwrap().push_front(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
