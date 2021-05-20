extern crate rand;

use std::cmp::{max, min};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::Rng;

// fn main() {
//     let mut rng = rand::thread_rng();

//     let n1: u8 = rng.gen();
//     let n2: u16 = rng.gen();

// TODO: добавить размышления

fn main() {
    const PHILOSOPHERS_NUMBER: usize = 5;
    const MAX_SECONDS_TO_EAT: u64 = 1;
    const MAX_SECONDS_TO_THINK: u64 = 1;

    let mut children = Vec::with_capacity(PHILOSOPHERS_NUMBER);

    let forks_taken = Arc::new(Mutex::new(HashSet::with_capacity(PHILOSOPHERS_NUMBER)));

    for n in 0..PHILOSOPHERS_NUMBER {
        let fork_arc = forks_taken.clone();
        children.push(thread::spawn(move || -> ! {
            let current_fork_number = n;
            let next_fork_number = (n + 1) % PHILOSOPHERS_NUMBER;

            let fork_to_take_first = min(current_fork_number, next_fork_number);
            let fork_to_take_last = max(current_fork_number, next_fork_number);

            // let mut rng = rand::thread_rng();

            loop {
                // цикл раздумий и еды...
                // println!("Philosopher#{} is thinking...", n);
                // thread::sleep(Duration::from_secs(rng.gen_range(0, MAX_SECONDS_TO_THINK) as u64));

                // блокируем сет вилок, чтобы узнать, а можно ли перекусить
                let mut forks_taken = fork_arc.lock().unwrap();
                if !forks_taken.contains(&fork_to_take_first)
                    && !forks_taken.contains(&fork_to_take_last)
                {
                    // да, можно кушать!
                    forks_taken.insert(fork_to_take_first);
                    println!("Philosopher#{} took fork#{}", n, fork_to_take_first);

                    forks_taken.insert(fork_to_take_last);
                    println!("Philosopher#{} took fork#{}", n, fork_to_take_last);

                    // освобождаем блокировку сета вилок
                    drop(forks_taken);

                    // едим...
                    println!("Philosopher#{} is eating.", n);
                    // thread::sleep(Duration::from_secs(
                    //     rng.gen_range(0, MAX_SECONDS_TO_EAT) as u64
                    // ));

                    // снова блокируем сет вилок
                    forks_taken = fork_arc.lock().unwrap();
                    forks_taken.remove(&fork_to_take_last);
                    drop(forks_taken);

                    forks_taken = fork_arc.lock().unwrap();
                    forks_taken.remove(&fork_to_take_first);
                    drop(forks_taken);
                } else {
                    drop(forks_taken);
                }
                thread::sleep(Duration::from_secs(1));
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
