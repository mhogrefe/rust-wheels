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
        if (i.leading_zeros() as u64) < j {
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
    pub fn new(size: usize) -> ZOrderTupleIndices {
        let mut v = Vec::new();
        v.resize(size, 0);
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
}

pub struct LogPairsFromSingle<I: Iterator>
    where I::Item: Clone
{
    xs: CachedIterator<I>,
    i: LogPairIndices,
    stop_checking_size: bool,
    max_indices: Option<LogPairIndices>,
}

impl<I: Iterator> Iterator for LogPairsFromSingle<I>
    where I::Item: Clone
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
    where I::Item: Clone
{
    LogPairsFromSingle {
        xs: CachedIterator::new(xs),
        i: LogPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct LogPairs<I: Iterator, J: Iterator>
    where I::Item: Clone,
          J::Item: Clone
{
    xs: CachedIterator<I>,
    ys: CachedIterator<J>,
    i: LogPairIndices,
    stop_checking_size: bool,
    max_indices: Option<LogPairIndices>,
}

impl<I: Iterator, J: Iterator> Iterator for LogPairs<I, J>
    where I::Item: Clone,
          J::Item: Clone
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
    where I::Item: Clone,
          J::Item: Clone
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
    where I::Item: Clone
{
    xs: CachedIterator<I>,
    i: SqrtPairIndices,
    stop_checking_size: bool,
    max_indices: Option<SqrtPairIndices>,
}

impl<I: Iterator> Iterator for SqrtPairsFromSingle<I>
    where I::Item: Clone
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
    where I::Item: Clone
{
    SqrtPairsFromSingle {
        xs: CachedIterator::new(xs),
        i: SqrtPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct SqrtPairs<I: Iterator, J: Iterator>
    where I::Item: Clone,
          J::Item: Clone
{
    xs: CachedIterator<I>,
    ys: CachedIterator<J>,
    i: SqrtPairIndices,
    stop_checking_size: bool,
    max_indices: Option<SqrtPairIndices>,
}

impl<I: Iterator, J: Iterator> Iterator for SqrtPairs<I, J>
    where I::Item: Clone,
          J::Item: Clone
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
    where I::Item: Clone,
          J::Item: Clone
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

        impl<I: Iterator> Iterator for $struct_name<I>
            where I::Item: Clone
        {
            type Item = $repeated_tuple;

            fn next(&mut self) -> Option<$repeated_tuple> {
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

exhaustive_tuple_from_single!(2,
                              (I::Item, I::Item),
                              ExhaustivePairsFromSingle,
                              exhaustive_pairs_from_single,
                              [0, ox],
                              [1, oy]);
exhaustive_tuple_from_single!(3,
                              (I::Item, I::Item, I::Item),
                              ExhaustiveTriplesFromSingle,
                              exhaustive_triples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz]);
exhaustive_tuple_from_single!(4,
                              (I::Item, I::Item, I::Item, I::Item),
                              ExhaustiveQuadruplesFromSingle,
                              exhaustive_quadruples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz],
                              [3, ow]);
exhaustive_tuple_from_single!(5,
                              (I::Item, I::Item, I::Item, I::Item, I::Item),
                              ExhaustiveQuintuplesFromSingle,
                              exhaustive_quintuples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz],
                              [3, ow],
                              [4, ov]);
exhaustive_tuple_from_single!(6,
                              (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
                              ExhaustiveSextuplesFromSingle,
                              exhaustive_sextuples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz],
                              [3, ow],
                              [4, ov],
                              [5, ou]);
exhaustive_tuple_from_single!(7,
                              (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
                              ExhaustiveSeptuplesFromSingle,
                              exhaustive_septuples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz],
                              [3, ow],
                              [4, ov],
                              [5, ou],
                              [6, ot]);
exhaustive_tuple_from_single!(8,
                              (I::Item,
                               I::Item,
                               I::Item,
                               I::Item,
                               I::Item,
                               I::Item,
                               I::Item,
                               I::Item),
                              ExhaustiveOctuplesFromSingle,
                              exhaustive_octuples_from_single,
                              [0, ox],
                              [1, oy],
                              [2, oz],
                              [3, ow],
                              [4, ov],
                              [5, ou],
                              [6, ot],
                              [7, os]);

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

            fn next(&mut self) -> Option<($($it_type::Item),*)> {
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

exhaustive_tuple!(2,
                  ExhaustivePairs,
                  exhaustive_pairs,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy]);
exhaustive_tuple!(3,
                  ExhaustiveTriples,
                  exhaustive_triples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz]);
exhaustive_tuple!(4,
                  ExhaustiveQuadruples,
                  exhaustive_quadruples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz],
                  [3, L, ws, ws, ow]);
exhaustive_tuple!(5,
                  ExhaustiveQuintuples,
                  exhaustive_quintuples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz],
                  [3, L, ws, ws, ow],
                  [4, M, vs, vs, ov]);
exhaustive_tuple!(6,
                  ExhaustiveSextuples,
                  exhaustive_sextuples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz],
                  [3, L, ws, ws, ow],
                  [4, M, vs, vs, ov],
                  [5, N, us, us, ou]);
exhaustive_tuple!(7,
                  ExhaustiveSeptuples,
                  exhaustive_septuples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz],
                  [3, L, ws, ws, ow],
                  [4, M, vs, vs, ov],
                  [5, N, us, us, ou],
                  [6, O, ts, ts, ot]);
exhaustive_tuple!(8,
                  ExhaustiveOctuples,
                  exhaustive_octuples,
                  [0, I, xs, xs, ox],
                  [1, J, ys, ys, oy],
                  [2, K, zs, zs, oz],
                  [3, L, ws, ws, ow],
                  [4, M, vs, vs, ov],
                  [5, N, us, us, ou],
                  [6, O, ts, ts, ot],
                  [7, P, ss, ss, os]);

pub struct RandomPairsFromSingle<I: Iterator>(I);

impl<I: Iterator> Iterator for RandomPairsFromSingle<I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        let x = self.0.next().unwrap();
        let y = self.0.next().unwrap();
        Some((x, y))
    }
}

//TODO test
pub fn random_pairs_from_single<I: Iterator>(xs: I) -> RandomPairsFromSingle<I> {
    RandomPairsFromSingle(xs)
}

pub struct RandomPairs<I: Iterator, J: Iterator> {
    xs: I,
    ys: J,
}

impl<I: Iterator, J: Iterator> Iterator for RandomPairs<I, J> {
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        let x = self.xs.next().unwrap();
        let y = self.ys.next().unwrap();
        Some((x, y))
    }
}

//TODO tst
pub fn random_pairs<I: Iterator, J: Iterator>(seed: &[u32],
                                              xs_gen: &Fn(&[u32]) -> I,
                                              ys_gen: &Fn(&[u32]) -> J)
                                              -> RandomPairs<I, J> {
    RandomPairs {
        xs: xs_gen(&scramble(seed, "xs")),
        ys: ys_gen(&scramble(seed, "ys")),
    }
}
