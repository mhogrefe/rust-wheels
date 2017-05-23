use gnuplot::{AxesCommon, Figure, Caption, Color};
use stats::{mean, median};
use std::collections::{BTreeMap, HashMap};
use time::precise_time_ns;

#[allow(unused_variables)]
pub fn benchmark_2<I: Iterator, IR, JITEM, JR>(xs: I,
                                               f: &Fn(I::Item) -> IR,
                                               g: &Fn(JITEM) -> JR,
                                               x_to_y: &Fn(&I::Item) -> JITEM,
                                               x_size: &Fn(&I::Item) -> usize,
                                               limit: usize,
                                               f_name: &str,
                                               g_name: &str,
                                               title: &str,
                                               x_axis_label: &str,
                                               y_axis_label: &str,
                                               file_name: &str)
    where I::Item: Clone
{
    let reps = 100;
    let min_bucket_size = 10;

    let mut xs_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut ys_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    for x in xs.take(limit) {
        let size = x_size(&x);
        let mut x_durations = Vec::new();
        let mut y_durations = Vec::new();
        for _ in 0..reps {
            let x_new = x.clone();
            let y = x_to_y(&x);
            let start_time = precise_time_ns();
            let result = f(x_new);
            let end_time = precise_time_ns();
            x_durations.push(end_time - start_time);

            let start_time = precise_time_ns();
            let result = g(y);
            let end_time = precise_time_ns();
            y_durations.push(end_time - start_time);
        }
        let x_duration = median(x_durations.iter().cloned()).unwrap();
        let y_duration = median(y_durations.iter().cloned()).unwrap();
        xs_durations_map.entry(size).or_insert_with(Vec::new).push(x_duration);
        ys_durations_map.entry(size).or_insert_with(Vec::new).push(y_duration);
    }
    let mut xs_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in xs_durations_map {
        if durations.len() >= min_bucket_size {
            xs_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }
    let mut ys_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in ys_durations_map {
        if durations.len() >= min_bucket_size {
            ys_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }

    let xs_sizes: Vec<usize> = xs_median_durations_map.iter().map(|entry| *entry.0).collect();
    let xs_durations: Vec<u64> = xs_median_durations_map.iter().map(|entry| *entry.1).collect();
    let ys_sizes: Vec<usize> = ys_median_durations_map.iter().map(|entry| *entry.0).collect();
    let ys_durations: Vec<u64> = ys_median_durations_map.iter().map(|entry| *entry.1).collect();
    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(title, &[]);
        axes.set_x_label(x_axis_label, &[]);
        axes.set_y_label(y_axis_label, &[]);
        axes.lines(&xs_sizes, &xs_durations, &[Caption(f_name), Color("green")]);
        axes.lines(&ys_sizes, &ys_durations, &[Caption(g_name), Color("blue")]);
    }
    fg.echo_to_file(file_name);
}

#[allow(unused_variables)]
pub fn benchmark_3<I: Iterator, IR, JITEM, JR, KITEM, KR>(xs: I,
                                                          f: &Fn(I::Item) -> IR,
                                                          g: &Fn(JITEM) -> JR,
                                                          h: &Fn(KITEM) -> KR,
                                                          x_to_y: &Fn(&I::Item) -> JITEM,
                                                          x_to_z: &Fn(&I::Item) -> KITEM,
                                                          x_size: &Fn(&I::Item) -> usize,
                                                          limit: usize,
                                                          f_name: &str,
                                                          g_name: &str,
                                                          h_name: &str,
                                                          title: &str,
                                                          x_axis_label: &str,
                                                          y_axis_label: &str,
                                                          file_name: &str)
    where I::Item: Clone
{
    let reps = 100;
    let min_bucket_size = 10;

    let mut xs_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut ys_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut zs_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    for x in xs.take(limit) {
        let size = x_size(&x);
        let mut x_durations = Vec::new();
        let mut y_durations = Vec::new();
        let mut z_durations = Vec::new();
        for _ in 0..reps {
            let x_new = x.clone();
            let y = x_to_y(&x);
            let z = x_to_z(&x);
            let start_time = precise_time_ns();
            let result = f(x_new);
            let end_time = precise_time_ns();
            x_durations.push(end_time - start_time);

            let start_time = precise_time_ns();
            let result = g(y);
            let end_time = precise_time_ns();
            y_durations.push(end_time - start_time);

            let start_time = precise_time_ns();
            let result = h(z);
            let end_time = precise_time_ns();
            z_durations.push(end_time - start_time);
        }
        let x_duration = median(x_durations.iter().cloned()).unwrap();
        let y_duration = median(y_durations.iter().cloned()).unwrap();
        let z_duration = median(z_durations.iter().cloned()).unwrap();
        xs_durations_map.entry(size).or_insert_with(Vec::new).push(x_duration);
        ys_durations_map.entry(size).or_insert_with(Vec::new).push(y_duration);
        zs_durations_map.entry(size).or_insert_with(Vec::new).push(z_duration);
    }
    let mut xs_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in xs_durations_map {
        if durations.len() >= min_bucket_size {
            xs_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }
    let mut ys_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in ys_durations_map {
        if durations.len() >= min_bucket_size {
            ys_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }
    let mut zs_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in zs_durations_map {
        if durations.len() >= min_bucket_size {
            zs_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }

    let xs_sizes: Vec<usize> = xs_median_durations_map.iter().map(|entry| *entry.0).collect();
    let xs_durations: Vec<u64> = xs_median_durations_map.iter().map(|entry| *entry.1).collect();
    let ys_sizes: Vec<usize> = ys_median_durations_map.iter().map(|entry| *entry.0).collect();
    let ys_durations: Vec<u64> = ys_median_durations_map.iter().map(|entry| *entry.1).collect();
    let zs_sizes: Vec<usize> = zs_median_durations_map.iter().map(|entry| *entry.0).collect();
    let zs_durations: Vec<u64> = zs_median_durations_map.iter().map(|entry| *entry.1).collect();
    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(title, &[]);
        axes.set_x_label(x_axis_label, &[]);
        axes.set_y_label(y_axis_label, &[]);
        axes.lines(&xs_sizes, &xs_durations, &[Caption(f_name), Color("green")]);
        axes.lines(&ys_sizes, &ys_durations, &[Caption(g_name), Color("blue")]);
        axes.lines(&zs_sizes, &zs_durations, &[Caption(h_name), Color("red")]);
    }
    fg.echo_to_file(file_name);
}
