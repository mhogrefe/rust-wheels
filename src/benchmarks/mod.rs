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
                $color: expr,
            ]
        ),*
    ) => {
        pub struct $struct_name<'a, I: Iterator, IR, T, $($ind_type, $dep_type),*>
            where I::Item: 'a, IR: 'a, T: 'a, $($ind_type: 'a, $dep_type: 'a),*
        {
            pub xs: I,
            pub function_f: &'a Fn(T) -> IR,
            $(pub $fun: &'a Fn($ind_type) -> $dep_type),*,
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
        "purple",
    ]
);
benchmark!(
    BenchmarkOptions6,
    benchmark_6,
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
        x_1,
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
        x_2,
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
        x_3,
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
        x_4,
        v_sizes,
        "purple",
    ],
    [
        NITEM,
        NR,
        function_k,
        u_cons,
        k_name,
        us_durations_map,
        us_median_durations_map,
        us_durations,
        u_duration,
        x_5,
        u_sizes,
        "orange",
    ]
);
