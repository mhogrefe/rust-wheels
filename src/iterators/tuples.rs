use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;

use iterators::common::scramble;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LogPairIndices(u64);

impl LogPairIndices {
    pub(crate) fn new() -> LogPairIndices {
        LogPairIndices(1)
    }

    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }

    pub(crate) fn indices(&self) -> (usize, usize) {
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
pub(crate) struct SqrtPairIndices {
    pub x: u64,
    pub y: u64,
}

impl SqrtPairIndices {
    pub(crate) fn new() -> SqrtPairIndices {
        SqrtPairIndices { x: 0, y: 0 }
    }

    pub(crate) fn increment(&mut self) {
        let mut ix = 0;
        let mut iy = 0;
        loop {
            let mask = u64::power_of_2(iy);
            if self.y & mask != 0 {
                self.y &= !mask;
                iy += 1;
            } else {
                self.y |= mask;
                return;
            }
            for _ in 0..2 {
                let mask = u64::power_of_2(ix);
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
pub(crate) struct ZOrderTupleIndices(pub Vec<u64>);

impl ZOrderTupleIndices {
    pub(crate) fn new(size: u64) -> ZOrderTupleIndices {
        let mut v = Vec::new();
        v.resize(usize::exact_from(size), 0);
        ZOrderTupleIndices(v)
    }

    pub(crate) fn increment(&mut self) {
        for j in 0..64 {
            let mask = u64::power_of_2(j);
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
