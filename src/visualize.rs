use std::any::Any;

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
    for sample in 0..samples {
        let mut row = vec![];
        let num = start + step * sample;
        let (averaged_duration, _) = timer::timing_average(|| func(num), average_over);
        row.push(num as u128);
        row.push(averaged_duration);
        outputs.push(row);
    }
    export::export_data(export::make_data(outputs), path);
}
