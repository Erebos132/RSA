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
    for _ in 0..=average_over {
        func();
    }
    let mut result = func();
    let duration = start.elapsed();
    (duration.as_millis() / average_over as u128, result)
}

pub fn timing_stdev<F>(mut func: F, sample_size: usize) -> (u128, u128)
where
    F: FnMut(),
{
    let mut samples = vec![];
    for _ in 0..=sample_size {
        let start = time::Instant::now();
        func();
        samples.push(start.elapsed().as_millis() as i128);
    }
    let mean = samples.iter().sum::<i128>() / samples.len() as i128;
    let mut theta = 0;
    for sample in samples {
        theta += (sample - mean).pow(2);
    }
    theta /= (sample_size as i128 - 1);
    theta = (theta as f64).sqrt() as i128;
    return (mean as u128, theta as u128);
}
