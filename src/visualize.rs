use kdam::tqdm;

pub mod export;
pub mod timer;

// The library and basic functionality of creating graphs (csv files)

pub fn create_graph<F>(
    mut func: F,
    samples: usize,
    start: usize,
    step: usize,
    average_over: usize,
    path: &str,
) where
    F: FnMut(usize),
{
    let mut outputs = vec![];
    for sample in tqdm!(0..samples) {
        let mut row = vec![];
        let num = start + step * sample;
        let (averaged_duration, _) = timer::timing_average(|| func(num), average_over);
        row.push(num as u128);
        row.push(averaged_duration);
        outputs.push(row);
    }
    export::export_data(export::make_data(outputs), path);
}

pub fn create_graph_stdev<F>(
    mut func: F,
    samples: usize,
    start: usize,
    step: usize,
    average_over: usize,
    path: &str,
) where
    F: FnMut(usize),
{
    let mut outputs = vec![];
    for sample in tqdm!(0..samples) {
        let mut row = vec![];
        let num = start + step * sample;
        let (averaged_duration, stdev) = timer::timing_stdev(|| func(num), average_over);
        row.push(num as u128);
        row.push(averaged_duration);
        row.push(stdev);
        outputs.push(row);
    }
    export::export_data(export::make_data(outputs), path);
}

// Written by Chatgpt
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub fn create_graph_stdev_threaded<F>(
    func: F,
    samples: usize,
    start: usize,
    step: usize,
    average_over: usize,
    path: &str,
) where
    F: Fn(usize) + Send + Sync + 'static,
{
    const THREADS: usize = 16;

    let func = Arc::new(func);

    let (job_tx, job_rx) = mpsc::channel::<(usize, usize)>();
    let (res_tx, res_rx) = mpsc::channel::<(usize, Vec<u128>)>();

    let job_rx = Arc::new(Mutex::new(job_rx));

    // Spawn workers
    for _ in 0..THREADS {
        let job_rx = Arc::clone(&job_rx);
        let res_tx = res_tx.clone();
        let func = Arc::clone(&func);

        thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };

                let (idx, num) = match job {
                    Ok(j) => j,
                    Err(_) => break, // channel closed
                };

                let (avg, stdev) = timer::timing_stdev(|| func(num), average_over);

                let row = vec![num as u128, avg, stdev];
                let _ = res_tx.send((idx, row));
            }
        });
    }

    drop(res_tx);

    // Send jobs
    for i in 0..samples {
        let num = start + step * i;
        job_tx.send((i, num)).unwrap();
    }

    drop(job_tx); // close queue

    // Collect results
    let mut outputs = vec![Vec::new(); samples];
    for (idx, row) in res_rx {
        outputs[idx] = row;
    }

    export::export_data(export::make_data(outputs), path);
}
