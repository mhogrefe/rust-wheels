use gnuplot::{AxesCommon, Figure, Caption, Color};
use stats::{mean, median};
use std::collections::{BTreeMap, HashMap};
use time::precise_time_ns;

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
                $color: expr
            ]
        ),*
    ) => {
        pub struct $struct_name<'a, I: Iterator, IR, $($ind_type, $dep_type),*>
            where I::Item: 'a, IR: 'a, $($ind_type: 'a, $dep_type: 'a),*
        {
            pub xs: I,
            pub function_f: &'a Fn(I::Item) -> IR,
            $(pub $fun: &'a Fn($ind_type) -> $dep_type),*,
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
        pub fn $fn_name<I: Iterator, IR, $($ind_type, $dep_type),*>(
            options: $struct_name<I, IR, $($ind_type, $dep_type),*>
        )
            where I::Item: Clone
        {
            let reps = 10;
            let min_bucket_size = 5;

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
                    let x_new = x.clone();
                    let start_time = precise_time_ns();
                    let result = (options.function_f)(x_new);
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
                axes.set_title(options.title, &[]);
                axes.set_x_label(options.x_axis_label, &[]);
                axes.set_y_label(options.y_axis_label, &[]);
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

benchmark!(BenchmarkOptions2,
           benchmark_2,
           [JITEM,
            JR,
            function_g,
            x_to_y,
            g_name,
            ys_durations_map,
            ys_median_durations_map,
            ys_durations,
            y_duration,
            y,
            y_sizes,
            "blue"]);
benchmark!(BenchmarkOptions3,
           benchmark_3,
           [JITEM,
            JR,
            function_g,
            x_to_y,
            g_name,
            ys_durations_map,
            ys_median_durations_map,
            ys_durations,
            y_duration,
            y,
            y_sizes,
            "blue"],
           [KITEM,
            KR,
            function_h,
            x_to_z,
            h_name,
            zs_durations_map,
            zs_median_durations_map,
            zs_durations,
            z_duration,
            z,
            z_sizes,
            "red"]);
