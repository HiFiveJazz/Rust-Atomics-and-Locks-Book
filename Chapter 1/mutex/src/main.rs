use std::sync::Mutex;
use std::time::Duration;
use std::thread;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s|{
        for _ in 1..=10 {
            s.spawn(||{
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                println!("n: {n:?}");
                thread::sleep(Duration::from_secs(1));
            });
            // drop(guard);
        }
    });
}
