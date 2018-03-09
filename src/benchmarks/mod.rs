use gnuplot::{AxesCommon, Caption, Color, Figure};
use stats::{mean, median};
use std::collections::{BTreeMap, HashMap};
use time::precise_time_ns;

fn escape_label_string(s: &str) -> String {
    let mut escaped = String::new();
    for c in s.chars() {
        if c == '_' || c == '&' {
            escaped.push_str("\\\\");
        }
        escaped.push(c);
    }
    escaped
}

pub struct BenchmarkSeriesOptions<'a, T: 'a> {
    pub name: &'a str,
    pub function: &'a Fn(T),
    pub color: &'a str,
}

pub struct BenchmarkOptions<'a, I: Iterator>
where
    I::Item: 'a,
{
    pub generator: I,
    pub title: &'a str,
    pub limit: usize,
    pub bucketing_function: &'a Fn(&I::Item) -> usize,
    pub x_axis_label: &'a str,
    pub y_axis_label: &'a str,
    pub file_name: &'a str,
    pub series_options: Vec<BenchmarkSeriesOptions<'a, I::Item>>,
}

pub fn run_benchmark<I: Iterator>(options: BenchmarkOptions<I>)
where
    I::Item: Clone,
{
    let reps = 10;
    let min_bucket_size = 2;

    let mut durations_maps = Vec::new();
    for _ in 0..options.series_options.len() {
        durations_maps.push(HashMap::new());
    }
    for x in options.generator.take(options.limit) {
        let size = (options.bucketing_function)(&x);
        for (i, series) in options.series_options.iter().enumerate() {
            let mut durations_vec = Vec::new();
            for _ in 0..reps {
                let start_time = precise_time_ns();
                let x = x.clone();
                (series.function)(x);
                let end_time = precise_time_ns();
                durations_vec.push(end_time - start_time);
            }
            let median_duration = median(durations_vec.iter().cloned()).unwrap();
            durations_maps[i]
                .entry(size)
                .or_insert_with(Vec::new)
                .push(median_duration);
        }
    }
    let mut median_durations_maps = Vec::new();
    for durations_map in durations_maps {
        let mut median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
        for (&size, durations) in &durations_map {
            if durations.len() >= min_bucket_size {
                median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
            }
        }
        median_durations_maps.push(median_durations_map);
    }

    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(&escape_label_string(options.title), &[]);
        axes.set_x_label(&escape_label_string(options.x_axis_label), &[]);
        axes.set_y_label(&escape_label_string(options.y_axis_label), &[]);
        for (median_durations_map, options) in median_durations_maps
            .iter()
            .zip(options.series_options.iter())
        {
            let sizes: Vec<usize> = median_durations_map.iter().map(|entry| *entry.0).collect();
            let durations: Vec<u64> = median_durations_map.iter().map(|entry| *entry.1).collect();
            axes.lines(
                &sizes,
                &durations,
                &[
                    Caption(&escape_label_string(options.name)),
                    Color(options.color),
                ],
            );
        }
    }
    fg.echo_to_file(options.file_name);
}

macro_rules! benchmark {
    (
        $struct_name: ident, $fn_name: ident,
        $(
            [
                $ind_type: ident,
                $dep_type: ident,
                $fun: ident,
                $conv: ident,
                $name: ident,
                $durations_map: ident,
                $median_durations_map: ident,
                $durations: ident,
                $duration: ident,
                $elem: ident,
                $sizes: ident,
                $color: expr,
            ]
        ),*
    ) => {
        pub struct $struct_name<'a, I: Iterator, IR, T, $($ind_type, $dep_type),*>
            where I::Item: 'a, IR: 'a, T: 'a, $($ind_type: 'a, $dep_type: 'a),*
        {
            pub xs: I,
            pub function_f: &'a mut FnMut(T) -> IR,
            $(pub $fun: &'a mut FnMut($ind_type) -> $dep_type),*,
            pub x_cons: &'a Fn(&I::Item) -> T,
            $(pub $conv: &'a Fn(&I::Item) -> $ind_type),*,
            pub x_param: &'a Fn(&I::Item) -> usize,
            pub limit: usize,
            pub f_name: &'a str,
            $(pub $name: &'a str),*,
            pub title: &'a str,
            pub x_axis_label: &'a str,
            pub y_axis_label: &'a str,
            pub file_name: &'a str,
        }

        #[allow(unused_variables)]
        pub fn $fn_name<I: Iterator, IR, T, $($ind_type, $dep_type),*>(
            options: $struct_name<I, IR, T, $($ind_type, $dep_type),*>
        )
            where I::Item: Clone
        {
            let reps = 10;
            let min_bucket_size = 2;

            let mut xs_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
            $(
                let mut $durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
            )*
            for x in options.xs.take(options.limit) {
                let size = (options.x_param)(&x);
                let mut x_durations = Vec::new();
                $(
                    let mut $durations = Vec::new();
                )*
                for _ in 0..reps {
                    let xx = (options.x_cons)(&x);
                    let start_time = precise_time_ns();
                    let result = (options.function_f)(xx);
                    let end_time = precise_time_ns();
                    x_durations.push(end_time - start_time);
                    $(
                        let $elem = (options.$conv)(&x);
                        let start_time = precise_time_ns();
                        let result = (options.$fun)($elem);
                        let end_time = precise_time_ns();
                        $durations.push(end_time - start_time);
                    )*
                }
                let x_duration = median(x_durations.iter().cloned()).unwrap();
                $(
                    let $duration = median($durations.iter().cloned()).unwrap();
                )*
                xs_durations_map.entry(size).or_insert_with(Vec::new).push(x_duration);
                $(
                    $durations_map.entry(size).or_insert_with(Vec::new).push($duration);
                )*
            }
            let mut xs_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
            for (size, durations) in xs_durations_map {
                if durations.len() >= min_bucket_size {
                    xs_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
                }
            }
            $(
                let mut $median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
                for (size, durations) in $durations_map {
                    if durations.len() >= min_bucket_size {
                        $median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
                    }
                }
            )*

            let xs_sizes: Vec<usize> =
                xs_median_durations_map.iter().map(|entry| *entry.0).collect();
            let xs_durations: Vec<u64> =
                xs_median_durations_map.iter().map(|entry| *entry.1).collect();
            $(
                let $sizes: Vec<usize> =
                    $median_durations_map.iter().map(|entry| *entry.0).collect();
                let $durations: Vec<u64> =
                    $median_durations_map.iter().map(|entry| *entry.1).collect();
            )*
            let mut fg = Figure::new();
            {
                let axes = fg.axes2d();
                axes.set_title(&escape_label_string(options.title), &[]);
                axes.set_x_label(&escape_label_string(options.x_axis_label), &[]);
                axes.set_y_label(&escape_label_string(options.y_axis_label), &[]);
                axes.lines(&xs_sizes,
                           &xs_durations,
                           &[Caption(options.f_name), Color("green")]);
                $(
                    axes.lines(&$sizes,
                               &$durations,
                               &[Caption(options.$name), Color($color)]);
                )*
            }
            fg.echo_to_file(options.file_name);
        }
    }
}

pub struct BenchmarkOptions1<'a, I: Iterator, IR, T>
where
    I::Item: 'a,
    IR: 'a,
    T: 'a,
{
    pub xs: I,
    pub function_f: &'a mut FnMut(T) -> IR,
    pub x_cons: &'a Fn(&I::Item) -> T,
    pub x_param: &'a Fn(&I::Item) -> usize,
    pub limit: usize,
    pub f_name: &'a str,
    pub title: &'a str,
    pub x_axis_label: &'a str,
    pub y_axis_label: &'a str,
    pub file_name: &'a str,
}

#[allow(unused_variables)]
pub fn benchmark_1<I: Iterator, IR, T>(options: BenchmarkOptions1<I, IR, T>)
where
    I::Item: Clone,
{
    let reps = 10;
    let min_bucket_size = 2;

    let mut xs_durations_map: HashMap<usize, Vec<f64>> = HashMap::new();
    for x in options.xs.take(options.limit) {
        let size = (options.x_param)(&x);
        let mut x_durations = Vec::new();
        for _ in 0..reps {
            let xx = (options.x_cons)(&x);
            let start_time = precise_time_ns();
            let result = (options.function_f)(xx);
            let end_time = precise_time_ns();
            x_durations.push(end_time - start_time);
        }
        let x_duration = median(x_durations.iter().cloned()).unwrap();
        xs_durations_map
            .entry(size)
            .or_insert_with(Vec::new)
            .push(x_duration);
    }
    let mut xs_median_durations_map: BTreeMap<usize, u64> = BTreeMap::new();
    for (size, durations) in xs_durations_map {
        if durations.len() >= min_bucket_size {
            xs_median_durations_map.insert(size, mean(durations.iter().cloned()) as u64);
        }
    }
    let xs_sizes: Vec<usize> = xs_median_durations_map
        .iter()
        .map(|entry| *entry.0)
        .collect();
    let xs_durations: Vec<u64> = xs_median_durations_map
        .iter()
        .map(|entry| *entry.1)
        .collect();
    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        axes.set_title(options.title, &[]);
        axes.set_x_label(options.x_axis_label, &[]);
        axes.set_y_label(options.y_axis_label, &[]);
        axes.lines(
            &xs_sizes,
            &xs_durations,
            &[Caption(options.f_name), Color("black")],
        );
    }
    fg.echo_to_file(options.file_name);
}

benchmark!(
    BenchmarkOptions2,
    benchmark_2,
    [
        JITEM,
        JR,
        function_g,
        y_cons,
        g_name,
        ys_durations_map,
        ys_median_durations_map,
        ys_durations,
        y_duration,
        y,
        y_sizes,
        "blue",
    ]
);
benchmark!(
    BenchmarkOptions3,
    benchmark_3,
    [
        JITEM,
        JR,
        function_g,
        y_cons,
        g_name,
        ys_durations_map,
        ys_median_durations_map,
        ys_durations,
        y_duration,
        y,
        y_sizes,
        "blue",
    ],
    [
        KITEM,
        KR,
        function_h,
        z_cons,
        h_name,
        zs_durations_map,
        zs_median_durations_map,
        zs_durations,
        z_duration,
        z,
        z_sizes,
        "red",
    ]
);
benchmark!(
    BenchmarkOptions4,
    benchmark_4,
    [
        JITEM,
        JR,
        function_g,
        y_cons,
        g_name,
        ys_durations_map,
        ys_median_durations_map,
        ys_durations,
        y_duration,
        y,
        y_sizes,
        "blue",
    ],
    [
        KITEM,
        KR,
        function_h,
        z_cons,
        h_name,
        zs_durations_map,
        zs_median_durations_map,
        zs_durations,
        z_duration,
        z,
        z_sizes,
        "red",
    ],
    [
        LITEM,
        LR,
        function_i,
        w_cons,
        i_name,
        ws_durations_map,
        ws_median_durations_map,
        ws_durations,
        w_duration,
        w,
        w_sizes,
        "black",
    ]
);
benchmark!(
    BenchmarkOptions5,
    benchmark_5,
    [
        JITEM,
        JR,
        function_g,
        y_cons,
        g_name,
        ys_durations_map,
        ys_median_durations_map,
        ys_durations,
        y_duration,
        y,
        y_sizes,
        "blue",
    ],
    [
        KITEM,
        KR,
        function_h,
        z_cons,
        h_name,
        zs_durations_map,
        zs_median_durations_map,
        zs_durations,
        z_duration,
        z,
        z_sizes,
        "red",
    ],
    [
        LITEM,
        LR,
        function_i,
        w_cons,
        i_name,
        ws_durations_map,
        ws_median_durations_map,
        ws_durations,
        w_duration,
        w,
        w_sizes,
        "black",
    ],
    [
        MITEM,
        MR,
        function_j,
        v_cons,
        j_name,
        vs_durations_map,
        vs_median_durations_map,
        vs_durations,
        v_duration,
        v,
        v_sizes,
        "orange",
    ]
);
