use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len(); // notice len is usize
        let sum = numbers.iter().sum::<usize>(); // this is to match the usize of len in the return calculation below
        sum / len // both vars need to be of the same type
    });
    
    let average = t.join().unwrap();
    println!("Average {average}");
}
