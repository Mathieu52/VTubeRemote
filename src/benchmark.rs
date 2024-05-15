#![allow(unused)]

use std::future::Future;
use std::time::{Duration, Instant};

async fn benchmark_async_function<F, Fut, T, E>(task: F, count: u32, exclude_failures: bool) -> (Duration, u32, u32)
    where
        F: FnOnce() -> Fut + Copy,
        Fut: Future<Output=Result<T, E>>
{
    let mut failure_count = 0_u32;
    let mut success_count = 0_u32;
    let mut total_elapsed_time = Duration::ZERO;

    for _ in 0..count {
        let start_time = Instant::now();

        match task().await {
            Ok(_) => {
                success_count += 1;
                total_elapsed_time += start_time.elapsed()
            }
            Err(_) => {
                failure_count += 1;
                if !exclude_failures {
                    total_elapsed_time += start_time.elapsed()
                }
            }
        }
    }

    let average_elapsed_time = total_elapsed_time / if exclude_failures { success_count } else { count };

    (average_elapsed_time, success_count, failure_count)
}