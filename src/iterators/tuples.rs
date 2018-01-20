use iterators::common::scramble;
use iterators::general::CachedIterator;

#[derive(Debug, Eq, PartialEq)]
pub struct LogPairIndices(u64);

impl LogPairIndices {
    pub fn new() -> LogPairIndices {
        LogPairIndices(1)
    }

    pub fn from_indices(i: usize, j: usize) -> Option<LogPairIndices> {
        let i = i as u64;
        let j = j as u64;
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
        ((self.0 >> (y + 1)) as usize, y as usize)
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
            let mask = 1 << iy;
            if self.y & mask != 0 {
                self.y &= !mask;
                iy += 1;
            } else {
                self.y |= mask;
                return;
            }
            for _ in 0..2 {
                let mask = 1 << ix;
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
        v.resize(size as usize, 0);
        ZOrderTupleIndices(v)
    }

    pub fn increment(&mut self) {
        for j in 0..64 {
            let mask = 1 << j;
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
            let i = self.i.x as usize;
            let j = self.i.y as usize;
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
                    self.max_indices = Some(SqrtPairIndices {
                        x: size as u64 - 1,
                        y: size as u64 - 1,
                    });
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
            let i = self.i.x as usize;
            let j = self.i.y as usize;
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
                            x: xs_size as u64 - 1,
                            y: ys_size as u64 - 1,
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
                        let $opt_elem = self.xs.get(self.i.0[$index] as usize);
                        if $opt_elem.is_none() {
                            self.i.increment();
                            continue;
                        }
                    )*
                    if !self.stop_checking_size {
                        if let Some(size) = self.xs.currently_known_size() {
                            let size = size as u64;
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
                        let $opt_elem = self.$cached_it.get(self.i.0[$index] as usize);
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
                                $(self.$cached_it.currently_known_size().unwrap() as u64 - 1),*
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

macro_rules! lex_tuple {
    (
        $struct_name: ident,
        $fn_name: ident,
        $last_it_type: ident,
        $last_it: ident,
        $last_it_elem: ident,
        $last_it_init: ident,
        $(
            [
                $it_type: ident,
                $it: ident,
                $it_elem: ident,
                $it_init: ident,
                $rev_it: ident,
                $rev_it_elem: ident,
                $rev_it_init: ident
            ]
        ),*
    ) => {
        pub struct $struct_name<I: Iterator, $($it_type: Iterator,)* $last_it_type> {
            x: Option<I::Item>,
            $($it_elem: Option<$it_type::Item>,)*
            xs: I,
            $($it: $it_type,)*
            $last_it: $last_it_type,
            $($it_init: $it_type,)*
            $last_it_init: $last_it_type,
        }

        impl<I: Iterator, $($it_type: Iterator,)* $last_it_type: Iterator>
            $struct_name<I, $($it_type,)* $last_it_type>
                where I::Item: Clone $(,$it_type::Item: Clone)* {
            fn current(&self, last: Option<$last_it_type::Item>)
                -> Option<(I::Item, $($it_type::Item,)* $last_it_type::Item)> {
                Some((self.x.clone().unwrap(), $(self.$it_elem.clone().unwrap(),)* last.unwrap()))
            }
        }

        impl<I: Iterator, $($it_type: Clone + Iterator,)* $last_it_type: Clone + Iterator> Iterator
            for $struct_name<I, $($it_type,)* $last_it_type>
                where I::Item: Clone $(,$it_type::Item: Clone)* {
            type Item = (I::Item, $($it_type::Item,)* $last_it_type::Item);

            fn next(&mut self) -> Option<Self::Item> {
                if self.x.is_none() {
                    return None;
                }
                let mut $last_it_elem = self.$last_it.next();
                if $last_it_elem.is_some() {
                    return self.current($last_it_elem);
                }
                self.$last_it = self.$last_it_init.clone();
                $last_it_elem = self.$last_it.next();
                if $last_it_elem.is_none() {
                    self.x = None;
                    return None;
                }

                $(
                    self.$rev_it_elem = self.$rev_it.next();
                    if self.$rev_it_elem.is_some() {
                        return self.current($last_it_elem);
                    }
                    self.$rev_it = self.$rev_it_init.clone();
                    self.$rev_it_elem = self.$rev_it.next();
                    if self.$rev_it_elem.is_none() {
                        self.x = None;
                        return None;
                    }
                )*

                self.x = self.xs.next();
                if self.x.is_some() {
                    return self.current($last_it_elem);
                }
                None
            }
        }

        //TODO test
        #[allow(unknown_lints, too_many_arguments, many_single_char_names)]
        pub fn
            $fn_name<I: Iterator, $($it_type: Clone + Iterator,)* $last_it_type: Clone + Iterator>
            (mut xs: I, $(mut $it: $it_type,)* $last_it: $last_it_type) ->
            $struct_name<I, $($it_type,)* $last_it_type>
                where $($it_type::Item: Clone,)* $last_it_type::Item: Clone {
            let x = xs.next();
            $(
                let $it_init = $it.clone();
                let $it_elem = $it.next();
            )*
            $struct_name {
                x: x,
                $($it_elem: $it_elem,)*
                xs: xs,
                $($it: $it,)*
                $last_it: $last_it.clone(),
                $($it_init: $it_init,)*
                $last_it_init: $last_it
            }
        }
    }
}

lex_tuple!(LexPairs, lex_pairs, J, ys, y, ys_init,);
lex_tuple!(
    LexTriples,
    lex_triples,
    K,
    zs,
    z,
    zs_init,
    [J, ys, y, ys_init, ys, y, ys_init]
);
lex_tuple!(
    LexQuadruples,
    lex_quadruples,
    L,
    ws,
    w,
    ws_init,
    [J, ys, y, ys_init, zs, z, zs_init],
    [K, zs, z, zs_init, ys, y, ys_init]
);
lex_tuple!(
    LexQuintuples,
    lex_quintuples,
    M,
    vs,
    v,
    vs_init,
    [J, ys, y, ys_init, ws, w, ws_init],
    [K, zs, z, zs_init, zs, z, zs_init],
    [L, ws, w, ws_init, ys, y, ys_init]
);
lex_tuple!(
    LexSextuples,
    lex_sextuples,
    N,
    us,
    u,
    us_init,
    [J, ys, y, ys_init, vs, v, vs_init],
    [K, zs, z, zs_init, ws, w, ws_init],
    [L, ws, w, ws_init, zs, z, zs_init],
    [M, vs, v, vs_init, ys, y, ys_init]
);
lex_tuple!(
    LexSeptuples,
    lex_septuples,
    O,
    ts,
    t,
    ts_init,
    [J, ys, y, ys_init, us, u, us_init],
    [K, zs, z, zs_init, vs, v, vs_init],
    [L, ws, w, ws_init, ws, w, ws_init],
    [M, vs, v, vs_init, zs, z, zs_init],
    [N, us, u, us_init, ys, y, ys_init]
);
lex_tuple!(
    LexOctuples,
    lex_octuples,
    P,
    ss,
    s,
    ss_init,
    [J, ys, y, ys_init, ts, t, ts_init],
    [K, zs, z, zs_init, us, u, us_init],
    [L, ws, w, ws_init, vs, v, vs_init],
    [M, vs, v, vs_init, ws, w, ws_init],
    [N, us, u, us_init, zs, z, zs_init],
    [O, ts, t, ts_init, ys, y, ys_init]
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
    [x],
    [y],
    [z],
    [w],
    [v]
);
random_tuple_from_single!(
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    RandomSextuplesFromSingle,
    random_sextuples_from_single,
    [x],
    [y],
    [z],
    [w],
    [v],
    [u]
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
    [x],
    [y],
    [z],
    [w],
    [v],
    [u],
    [t]
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
    [x],
    [y],
    [z],
    [w],
    [v],
    [u],
    [t],
    [s]
);

macro_rules! random_tuple {
    (
        $struct_name: ident, $fn_name: ident,
        $(
            [ $it_type: ident, $it_gen: ident, $cached_it: ident, $elem: ident, $it_name: expr ]
        ),*
    ) => {
        pub struct $struct_name<$($it_type: Iterator),*> {
            $(
                $cached_it: $it_type
            ),*
        }

        impl<$($it_type: Iterator),*> Iterator for $struct_name<$($it_type),*> {
            type Item = ($($it_type::Item),*);

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    let $elem = self.$cached_it.next().unwrap();
                )*
                Some(($($elem),*))
            }
        }

        //TODO test
        #[allow(unknown_lints, too_many_arguments)]
        pub fn $fn_name<$($it_type: Iterator),*>(seed: &[u32],
                                                      $($it_gen: &Fn(&[u32]) -> $it_type),*)
                                                      -> $struct_name<$($it_type),*> {
            $struct_name {
                $(
                    $cached_it: $it_gen(&scramble(seed, $it_name))
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
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"],
    [L, ws_gen, ws, w, "ws"],
    [M, vs_gen, vs, v, "vs"]
);
random_tuple!(
    RandomSextuples,
    random_sextuples,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"],
    [L, ws_gen, ws, w, "ws"],
    [M, vs_gen, vs, v, "vs"],
    [N, us_gen, us, u, "us"]
);
random_tuple!(
    RandomSeptuples,
    random_septuples,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"],
    [L, ws_gen, ws, w, "ws"],
    [M, vs_gen, vs, v, "vs"],
    [N, us_gen, us, u, "us"],
    [O, ts_gen, ts, t, "ts"]
);
random_tuple!(
    RandomOctuples,
    random_octuples,
    [I, xs_gen, xs, x, "xs"],
    [J, ys_gen, ys, y, "ys"],
    [K, zs_gen, zs, z, "zs"],
    [L, ws_gen, ws, w, "ws"],
    [M, vs_gen, vs, v, "vs"],
    [N, us_gen, us, u, "us"],
    [O, ts_gen, ts, t, "ts"],
    [P, ss_gen, ss, s, "ss"]
);
