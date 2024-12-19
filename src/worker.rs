use num_cpus;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use crate::hash;

pub fn run(n_zeros: u32, n_results: u32) {
    let num_threads = num_cpus::get();
    let shared_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let cloned_counter = Arc::clone(&shared_counter);

        let handle = thread::spawn(move || {
            let mut current_num = thread_id;

            loop {
                if cloned_counter.load(Ordering::Relaxed) >= n_results as usize {
                    break;
                }

                let hash_string = hash::get_hash(current_num);
                if hash::has_n_trailing_zeros(&hash_string, n_zeros) {
                    let result = cloned_counter.fetch_update(
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                        |counter| {
                            if counter < n_results as usize {
                                Some(counter + 1)
                            } else {
                                None
                            }
                        },
                    );

                    if let Ok(_) = result {
                        println!("{current_num}, {hash_string}");
                    }
                }

                current_num += num_threads;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
