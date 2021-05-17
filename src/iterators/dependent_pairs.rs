use iterators::adaptors::Concat;
use iterators::general::CachedIterator;
use iterators::tuples::{LogPairIndices, ZOrderTupleIndices};
use malachite_base::num::conversion::traits::ExactFrom;
use std::collections::HashMap;
use std::hash::Hash;

pub fn dependent_pairs<'a, I: Iterator + 'a, J: Iterator, F: 'a>(
    xs: I,
    f: F,
) -> Box<dyn Iterator<Item = (I::Item, J::Item)> + 'a>
where
    F: Fn(&I::Item) -> J,
    I::Item: Clone,
{
    Box::new(Concat::new(
        xs.map(move |x| f(&x).map(move |y| (x.clone(), y))),
    ))
}

pub struct RandomDependentPairs<I: Iterator, J: Iterator, F, T>
where
    F: Fn(&T, &I::Item) -> J,
{
    xs: I,
    f: F,
    data: T,
    x_to_ys: HashMap<I::Item, J>,
}

impl<I: Iterator, J: Iterator, F, T> Iterator for RandomDependentPairs<I, J, F, T>
where
    F: Fn(&T, &I::Item) -> J,
    I::Item: Clone + Eq + Hash,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        let x = self.xs.next().unwrap();
        let ys = self
            .x_to_ys
            .entry(x.clone())
            .or_insert((self.f)(&self.data, &x));
        Some((x, ys.next().unwrap()))
    }
}

pub fn random_dependent_pairs<I: Iterator, J: Iterator, F, T>(
    data: T,
    xs: I,
    f: F,
) -> RandomDependentPairs<I, J, F, T>
where
    F: Fn(&T, &I::Item) -> J,
    I::Item: Clone + Eq + Hash,
{
    RandomDependentPairs {
        xs,
        f,
        data,
        x_to_ys: HashMap::new(),
    }
}

macro_rules! exhaustive_dependent_pairs {
    (
        $struct_name:ident,
        $fn_name:ident,
        $index_type:ident,
        $index_ctor:expr,
        $x_index_fn:expr
    ) => {
        pub struct $struct_name<I: Iterator, J: Iterator, F, T>
        where
            I::Item: Clone,
        {
            f: F,
            xs: CachedIterator<I>,
            x_to_ys: HashMap<I::Item, J>,
            i: $index_type,
            data: T,
        }

        impl<I: Iterator, J: Iterator, F, T> Iterator for $struct_name<I, J, F, T>
        where
            F: Fn(&T, &I::Item) -> J,
            I::Item: Clone + Eq + Hash,
        {
            type Item = (I::Item, J::Item);

            fn next(&mut self) -> Option<(I::Item, J::Item)> {
                let xi = $x_index_fn(&self.i);
                let x = self.xs.get(xi).unwrap();
                self.i.increment();
                if let Some(ys) = self.x_to_ys.get_mut(&x) {
                    return Some((x, ys.next().unwrap()));
                }
                let mut ys = (self.f)(&self.data, &x);
                let y = ys.next().unwrap();
                self.x_to_ys.insert(x.clone(), ys);
                Some((x, y))
            }
        }

        pub fn $fn_name<I: Iterator, J: Iterator, F, T>(
            data: T,
            xs: I,
            f: F,
        ) -> $struct_name<I, J, F, T>
        where
            F: Fn(&T, &I::Item) -> J,
            I::Item: Clone + Eq + Hash,
        {
            $struct_name {
                f,
                xs: CachedIterator::new(xs),
                x_to_ys: HashMap::new(),
                i: $index_ctor,
                data,
            }
        }
    };
}

exhaustive_dependent_pairs!(
    ExhaustiveDependentPairsInfiniteLog,
    exhaustive_dependent_pairs_infinite_log,
    LogPairIndices,
    LogPairIndices::new(),
    |i: &LogPairIndices| i.indices().1
);
exhaustive_dependent_pairs!(
    ExhaustiveDependentPairsInfinite,
    exhaustive_dependent_pairs_infinite,
    ZOrderTupleIndices,
    ZOrderTupleIndices::new(2),
    |i: &ZOrderTupleIndices| usize::exact_from(i.0[1])
);
