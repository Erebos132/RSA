use std::time;

pub fn timing<F, R>(func: F) -> (u128, R)
where
    F: FnOnce() -> R,
{
    let start = time::Instant::now();
    let result = func();
    let duration = start.elapsed();
    (duration.as_millis(), result)
}

pub fn timing_average<F, R>(mut func: F, average_over: usize) -> (u128, R)
where
    F: FnMut() -> R,
{
    let start = time::Instant::now();
    for _ in 0..average_over - 1 {
        func();
    }
    let mut result = func();
    let duration = start.elapsed();
    (duration.as_millis() / average_over as u128, result)
}
