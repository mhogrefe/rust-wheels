use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

use iterators::common::scramble;
use iterators::general::CachedIterator;

#[derive(Debug, Eq, PartialEq)]
pub struct LogPairIndices(u64);

impl LogPairIndices {
    pub fn new() -> LogPairIndices {
        LogPairIndices(1)
    }

    pub fn from_indices(i: usize, j: usize) -> Option<LogPairIndices> {
        let i = u64::exact_from(i);
        let j = u64::exact_from(j);
        if i.leading_zeros() == 0 {
            return None;
        }
        let i = (i << 1) | 1;
        if u64::from(i.leading_zeros()) < j {
            None
        } else {
            Some(LogPairIndices(i << j))
        }
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn indices(&self) -> (usize, usize) {
        let y = self.0.trailing_zeros();
        (usize::exact_from(self.0 >> (y + 1)), usize::exact_from(y))
    }
}

impl Default for LogPairIndices {
    fn default() -> LogPairIndices {
        LogPairIndices(1)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SqrtPairIndices {
    pub x: u64,
    pub y: u64,
}

impl SqrtPairIndices {
    pub fn new() -> SqrtPairIndices {
        SqrtPairIndices { x: 0, y: 0 }
    }

    pub fn increment(&mut self) {
        let mut ix = 0;
        let mut iy = 0;
        loop {
            let mask = u64::power_of_two(iy);
            if self.y & mask != 0 {
                self.y &= !mask;
                iy += 1;
            } else {
                self.y |= mask;
                return;
            }
            for _ in 0..2 {
                let mask = u64::power_of_two(ix);
                if self.x & mask != 0 {
                    self.x &= !mask;
                    ix += 1;
                } else {
                    self.x |= mask;
                    return;
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ZOrderTupleIndices(pub Vec<u64>);

impl ZOrderTupleIndices {
    pub fn new(size: u64) -> ZOrderTupleIndices {
        let mut v = Vec::new();
        v.resize(usize::exact_from(size), 0);
        ZOrderTupleIndices(v)
    }

    pub fn increment(&mut self) {
        for j in 0..64 {
            let mask = u64::power_of_two(j);
            for i in (0..self.0.len()).rev() {
                if self.0[i] & mask != 0 {
                    self.0[i] &= !mask;
                } else {
                    self.0[i] |= mask;
                    return;
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

pub struct LogPairsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    xs: CachedIterator<I>,
    i: LogPairIndices,
    stop_checking_size: bool,
    max_indices: Option<LogPairIndices>,
}

impl<I: Iterator> Iterator for LogPairsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let (i, j) = self.i.indices();
            let ox = self.xs.get(i);
            if ox.is_none() {
                self.i.increment();
                continue;
            }
            let oy = self.xs.get(j);
            if oy.is_none() {
                self.i.increment();
                continue;
            }
            if !self.stop_checking_size {
                if let Some(size) = self.xs.currently_known_size() {
                    self.max_indices = LogPairIndices::from_indices(size - 1, size - 1);
                    self.stop_checking_size = true;
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn log_pairs_from_single<I: Iterator>(xs: I) -> LogPairsFromSingle<I>
where
    I::Item: Clone,
{
    LogPairsFromSingle {
        xs: CachedIterator::new(xs),
        i: LogPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct LogPairs<I: Iterator, J: Iterator>
where
    I::Item: Clone,
    J::Item: Clone,
{
    xs: CachedIterator<I>,
    ys: CachedIterator<J>,
    i: LogPairIndices,
    stop_checking_size: bool,
    max_indices: Option<LogPairIndices>,
}

impl<I: Iterator, J: Iterator> Iterator for LogPairs<I, J>
where
    I::Item: Clone,
    J::Item: Clone,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let (i, j) = self.i.indices();
            let ox = self.xs.get(i);
            if ox.is_none() {
                self.i.increment();
                continue;
            }
            let oy = self.ys.get(j);
            if oy.is_none() {
                self.i.increment();
                continue;
            }
            if !self.stop_checking_size {
                if let Some(xs_size) = self.xs.currently_known_size() {
                    if let Some(ys_size) = self.ys.currently_known_size() {
                        self.max_indices = LogPairIndices::from_indices(xs_size - 1, ys_size - 1);
                        self.stop_checking_size = true;
                    }
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn log_pairs<I: Iterator, J: Iterator>(xs: I, ys: J) -> LogPairs<I, J>
where
    I::Item: Clone,
    J::Item: Clone,
{
    LogPairs {
        xs: CachedIterator::new(xs),
        ys: CachedIterator::new(ys),
        i: LogPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct SqrtPairsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    xs: CachedIterator<I>,
    i: SqrtPairIndices,
    stop_checking_size: bool,
    max_indices: Option<SqrtPairIndices>,
}

impl<I: Iterator> Iterator for SqrtPairsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let i = usize::exact_from(self.i.x);
            let j = usize::exact_from(self.i.y);
            let ox = self.xs.get(i);
            if ox.is_none() {
                self.i.increment();
                continue;
            }
            let oy = self.xs.get(j);
            if oy.is_none() {
                self.i.increment();
                continue;
            }
            if !self.stop_checking_size {
                if let Some(size) = self.xs.currently_known_size() {
                    let size = u64::wrapping_from(size) - 1;
                    self.max_indices = Some(SqrtPairIndices { x: size, y: size });
                    self.stop_checking_size = true;
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn sqrt_pairs_from_single<I: Iterator>(xs: I) -> SqrtPairsFromSingle<I>
where
    I::Item: Clone,
{
    SqrtPairsFromSingle {
        xs: CachedIterator::new(xs),
        i: SqrtPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct SqrtPairs<I: Iterator, J: Iterator>
where
    I::Item: Clone,
    J::Item: Clone,
{
    xs: CachedIterator<I>,
    ys: CachedIterator<J>,
    i: SqrtPairIndices,
    stop_checking_size: bool,
    max_indices: Option<SqrtPairIndices>,
}

impl<I: Iterator, J: Iterator> Iterator for SqrtPairs<I, J>
where
    I::Item: Clone,
    J::Item: Clone,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let i = usize::exact_from(self.i.x);
            let j = usize::exact_from(self.i.y);
            let ox = self.xs.get(i);
            if ox.is_none() {
                self.i.increment();
                continue;
            }
            let oy = self.ys.get(j);
            if oy.is_none() {
                self.i.increment();
                continue;
            }
            if !self.stop_checking_size {
                if let Some(xs_size) = self.xs.currently_known_size() {
                    if let Some(ys_size) = self.ys.currently_known_size() {
                        self.max_indices = Some(SqrtPairIndices {
                            x: u64::wrapping_from(xs_size) - 1,
                            y: u64::wrapping_from(ys_size) - 1,
                        });
                        self.stop_checking_size = true;
                    }
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn sqrt_pairs<I: Iterator, J: Iterator>(xs: I, ys: J) -> SqrtPairs<I, J>
where
    I::Item: Clone,
    J::Item: Clone,
{
    SqrtPairs {
        xs: CachedIterator::new(xs),
        ys: CachedIterator::new(ys),
        i: SqrtPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

// TODO exhaustive_pairs_from_single(range_increasing(0, 0)) doesn't work
macro_rules! exhaustive_tuple_from_single {
    (
        $size: expr, $repeated_tuple: ty, $struct_name: ident, $fn_name: ident,
        $(
            [ $index: expr, $opt_elem: ident ]
        ),*
    ) => {
        pub struct $struct_name<I: Iterator>
            where I::Item: Clone
        {
            xs: CachedIterator<I>,
            i: ZOrderTupleIndices,
            stop_checking_size: bool,
            max_indices: Option<ZOrderTupleIndices>,
        }

        #[allow(unknown_lints, type_complexity)]
        impl<I: Iterator> Iterator for $struct_name<I>
            where I::Item: Clone
        {
            type Item = $repeated_tuple;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if self.max_indices.as_ref() == Some(&self.i) {
                        return None;
                    }
                    $(
                        let $opt_elem = self.xs.get(usize::exact_from(self.i.0[$index]));
                        if $opt_elem.is_none() {
                            self.i.increment();
                            continue;
                        }
                    )*
                    if !self.stop_checking_size {
                        if let Some(size) = self.xs.currently_known_size() {
                            let size = u64::wrapping_from(size);
                            let mut max_vec = Vec::new();
                            max_vec.resize($size, size - 1);
                            self.max_indices = Some(ZOrderTupleIndices(max_vec));
                            self.stop_checking_size = true;
                        }
                    }
                    self.i.increment();
                    return Some(($($opt_elem.unwrap()),*));
                }
            }
        }

        //TODO test
        pub fn $fn_name<I: Iterator>(xs: I) -> $struct_name<I>
            where I::Item: Clone
        {
            $struct_name {
                xs: CachedIterator::new(xs),
                i: ZOrderTupleIndices::new($size),
                stop_checking_size: false,
                max_indices: None,
            }
        }
    }
}

exhaustive_tuple_from_single!(
    2,
    (I::Item, I::Item),
    ExhaustivePairsFromSingle,
    exhaustive_pairs_from_single,
    [0, ox],
    [1, oy]
);
exhaustive_tuple_from_single!(
    3,
    (I::Item, I::Item, I::Item),
    ExhaustiveTriplesFromSingle,
    exhaustive_triples_from_single,
    [0, ox],
    [1, oy],
    [2, oz]
);
exhaustive_tuple_from_single!(
    4,
    (I::Item, I::Item, I::Item, I::Item),
    ExhaustiveQuadruplesFromSingle,
    exhaustive_quadruples_from_single,
    [0, ox],
    [1, oy],
    [2, oz],
    [3, ow]
);
exhaustive_tuple_from_single!(
    5,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    ExhaustiveQuintuplesFromSingle,
    exhaustive_quintuples_from_single,
    [0, ox],
    [1, oy],
    [2, oz],
    [3, ow],
    [4, ov]
);
exhaustive_tuple_from_single!(
    6,
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    ExhaustiveSextuplesFromSingle,
    exhaustive_sextuples_from_single,
    [0, ox],
    [1, oy],
    [2, oz],
    [3, ow],
    [4, ov],
    [5, ou]
);
exhaustive_tuple_from_single!(
    7,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
    ),
    ExhaustiveSeptuplesFromSingle,
    exhaustive_septuples_from_single,
    [0, ox],
    [1, oy],
    [2, oz],
    [3, ow],
    [4, ov],
    [5, ou],
    [6, ot]
);
exhaustive_tuple_from_single!(
    8,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
    ),
    ExhaustiveOctuplesFromSingle,
    exhaustive_octuples_from_single,
    [0, ox],
    [1, oy],
    [2, oz],
    [3, ow],
    [4, ov],
    [5, ou],
    [6, ot],
    [7, os]
);

macro_rules! exhaustive_tuple {
    (
        $size: expr, $struct_name: ident, $fn_name: ident,
        $(
            [ $index: expr, $it_type: ident, $it: ident, $cached_it: ident, $opt_elem: ident ]
        ),*
    ) => {
        pub struct $struct_name<$($it_type: Iterator),*> where $($it_type::Item: Clone),*
        {
            $($cached_it: CachedIterator<$it_type>),*,
            i: ZOrderTupleIndices,
            stop_checking_size: bool,
            max_indices: Option<ZOrderTupleIndices>,
        }

        impl<$($it_type: Iterator),*> Iterator for $struct_name<$($it_type),*>
            where $($it_type::Item: Clone),*
        {
            type Item = ($($it_type::Item),*);

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    if self.max_indices.as_ref() == Some(&self.i) {
                        return None;
                    }
                    $(
                        let $opt_elem =
                            self.$cached_it.get(usize::exact_from(self.i.0[$index]));
                        if $opt_elem.is_none() {
                            self.i.increment();
                            continue;
                        }
                    )*
                    if !self.stop_checking_size {
                        let mut all_sizes_available = true;
                        $(
                            if all_sizes_available &&
                                self.$cached_it.currently_known_size().is_none() {
                                all_sizes_available = false;
                            }
                        )*

                        if all_sizes_available {
                            self.max_indices = Some(ZOrderTupleIndices(vec![
                                $(u64::wrapping_from(
                                    self.$cached_it.currently_known_size().unwrap()) - 1
                                ),*
                            ]));
                            self.stop_checking_size = true;
                        }
                    }
                    self.i.increment();
                    return Some(($($opt_elem.unwrap()),*));
                }
            }
        }

        //TODO test
        #[allow(unknown_lints, too_many_arguments)]
        pub fn $fn_name<$($it_type: Iterator),*>($($it: $it_type),*) ->
            $struct_name<$($it_type),*> where $($it_type::Item: Clone),*
        {
            $struct_name {
                $($cached_it: CachedIterator::new($it)),*,
                i: ZOrderTupleIndices::new($size),
                stop_checking_size: false,
                max_indices: None,
            }
        }
    }
}

exhaustive_tuple!(
    2,
    ExhaustivePairs,
    exhaustive_pairs,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy]
);
exhaustive_tuple!(
    3,
    ExhaustiveTriples,
    exhaustive_triples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz]
);
exhaustive_tuple!(
    4,
    ExhaustiveQuadruples,
    exhaustive_quadruples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz],
    [3, L, ws, ws, ow]
);
exhaustive_tuple!(
    5,
    ExhaustiveQuintuples,
    exhaustive_quintuples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz],
    [3, L, ws, ws, ow],
    [4, M, vs, vs, ov]
);
exhaustive_tuple!(
    6,
    ExhaustiveSextuples,
    exhaustive_sextuples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz],
    [3, L, ws, ws, ow],
    [4, M, vs, vs, ov],
    [5, N, us, us, ou]
);
exhaustive_tuple!(
    7,
    ExhaustiveSeptuples,
    exhaustive_septuples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz],
    [3, L, ws, ws, ow],
    [4, M, vs, vs, ov],
    [5, N, us, us, ou],
    [6, O, ts, ts, ot]
);
exhaustive_tuple!(
    8,
    ExhaustiveOctuples,
    exhaustive_octuples,
    [0, I, xs, xs, ox],
    [1, J, ys, ys, oy],
    [2, K, zs, zs, oz],
    [3, L, ws, ws, ow],
    [4, M, vs, vs, ov],
    [5, N, us, us, ou],
    [6, O, ts, ts, ot],
    [7, P, ss, ss, os]
);

macro_rules! random_tuple_from_single {
    (
        $repeated_tuple: ty, $struct_name: ident, $fn_name: ident,
        $(
            [ $elem: ident ]
        ),*
    ) => {
        pub struct $struct_name<I: Iterator>(I);

        #[allow(unknown_lints, type_complexity)]
        impl<I: Iterator> Iterator for $struct_name<I> {
            type Item = $repeated_tuple;

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    let $elem = self.0.next().unwrap();
                )*
                Some(($($elem),*))
            }
        }

        //TODO test
        pub fn $fn_name<I: Iterator>(xs: I) -> $struct_name<I> {
            $struct_name(xs)
        }
    }
}

random_tuple_from_single!(
    (I::Item, I::Item),
    RandomPairsFromSingle,
    random_pairs_from_single,
    [x],
    [y]
);
random_tuple_from_single!(
    (I::Item, I::Item, I::Item),
    RandomTriplesFromSingle,
    random_triples_from_single,
    [x],
    [y],
    [z]
);
random_tuple_from_single!(
    (I::Item, I::Item, I::Item, I::Item),
    RandomQuadruplesFromSingle,
    random_quadruples_from_single,
    [x],
    [y],
    [z],
    [w]
);
random_tuple_from_single!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    RandomQuintuplesFromSingle,
    random_quintuples_from_single,
    [x_0],
    [x_1],
    [x_2],
    [x_3],
    [x_4]
);
random_tuple_from_single!(
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    RandomSextuplesFromSingle,
    random_sextuples_from_single,
    [x_0],
    [x_1],
    [x_2],
    [x_3],
    [x_4],
    [x_5]
);
random_tuple_from_single!(
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
    ),
    RandomSeptuplesFromSingle,
    random_septuples_from_single,
    [x_0],
    [x_1],
    [x_2],
    [x_3],
    [x_4],
    [x_5],
    [x_6]
);
random_tuple_from_single!(
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
    ),
    RandomOctuplesFromSingle,
    random_octuples_from_single,
    [x_0],
    [x_1],
    [x_2],
    [x_3],
    [x_4],
    [x_5],
    [x_6],
    [x_7]
);

macro_rules! random_tuple {
    (
        $struct_name: ident, $fn_name: ident,
        $(
            [ $it_type: ident, $it_gen: ident, $it: ident, $elem: ident, $it_name: expr ]
        ),*
    ) => {
        pub struct $struct_name<$($it_type: Iterator),*> {
            $(
                $it: $it_type
            ),*
        }

        impl<$($it_type: Iterator),*> Iterator for $struct_name<$($it_type),*> {
            type Item = ($($it_type::Item),*);

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    let $elem = self.$it.next().unwrap();
                )*
                Some(($($elem),*))
            }
        }

        //TODO test
        #[allow(unknown_lints, too_many_arguments)]
        pub fn $fn_name<$($it_type: Iterator),*>(seed: &[u32],
                                                      $($it_gen: &dyn Fn(&[u32]) -> $it_type),*)
                                                      -> $struct_name<$($it_type),*> {
            $struct_name {
                $(
                    $it: $it_gen(&scramble(seed, $it_name))
                ),*
            }
        }
    }
}

random_tuple!(
    RandomPairs,
    random_pairs,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"]
);
random_tuple!(
    RandomTriples,
    random_triples,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"]
);
random_tuple!(
    RandomQuadruples,
    random_quadruples,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"],
    [L, ws_gen, ws, w, "ws"]
);
random_tuple!(
    RandomQuintuples,
    random_quintuples,
    [I, xs_gen, xs, x_0, "xs"],
    [J, ys_gen, ys, x_1, "ys"],
    [K, zs_gen, zs, x_2, "zs"],
    [L, ws_gen, ws, x_3, "ws"],
    [M, vs_gen, vs, x_4, "vs"]
);
random_tuple!(
    RandomSextuples,
    random_sextuples,
    [I, xs_gen, xs, x_0, "xs"],
    [J, ys_gen, ys, x_1, "ys"],
    [K, zs_gen, zs, x_2, "zs"],
    [L, ws_gen, ws, x_3, "ws"],
    [M, vs_gen, vs, x_4, "vs"],
    [N, us_gen, us, x_5, "us"]
);
random_tuple!(
    RandomSeptuples,
    random_septuples,
    [I, xs_gen, xs, x_0, "xs"],
    [J, ys_gen, ys, x_1, "ys"],
    [K, zs_gen, zs, x_2, "zs"],
    [L, ws_gen, ws, x_3, "ws"],
    [M, vs_gen, vs, x_4, "vs"],
    [N, us_gen, us, x_5, "us"],
    [O, ts_gen, ts, x_6, "ts"]
);
random_tuple!(
    RandomOctuples,
    random_octuples,
    [I, xs_gen, xs, x_0, "xs"],
    [J, ys_gen, ys, x_1, "ys"],
    [K, zs_gen, zs, x_2, "zs"],
    [L, ws_gen, ws, x_3, "ws"],
    [M, vs_gen, vs, x_4, "vs"],
    [N, us_gen, us, x_5, "us"],
    [O, ts_gen, ts, x_6, "ts"],
    [P, ss_gen, ss, x_7, "ss"]
);
