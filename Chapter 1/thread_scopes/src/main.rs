use std::thread;

fn main() {
    let numbers = vec![1,2,3];

    // using a thread::scope means we don't have to move
    // values into the threads and they can be shared across
    // spawned threads.
    thread::scope(|s| {
        s.spawn(||{
            println!("length: {}", numbers.len());
        });
        s.spawn(||{
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
