use iterators::adaptors::Concat;
use iterators::general::CachedIterator;
use iterators::tuples::{LogPairIndices, SqrtPairIndices, ZOrderTupleIndices};
use std::collections::HashMap;
use std::hash::Hash;

pub fn dependent_pairs<'a, I: Iterator + 'a, J: Iterator, F: 'a>
    (xs: I,
     f: F)
     -> Box<Iterator<Item = (I::Item, J::Item)> + 'a>
    where F: Fn(&I::Item) -> J,
          I::Item: Clone
{
    Box::new(Concat::new(xs.map(move |x| f(&x).map(move |y| (x.clone(), y)))))
}

macro_rules! exhaustive_dependent_pairs {
    (
        $struct_name: ident,
        $fn_name: ident,
        $index_type: ident,
        $index_ctor: expr,
        $x_index_fn: expr
    ) => {
        pub struct $struct_name<I: Iterator, J: Iterator, F>
            where I::Item: Clone
        {
            f: F,
            xs: CachedIterator<I>,
            x_to_ys: HashMap<I::Item, J>,
            i: $index_type,
        }

        impl<I: Iterator, J: Iterator, F> Iterator for $struct_name<I, J, F>
            where F: Fn(&I::Item) -> J,
                  I::Item: Clone + Eq + Hash
        {
            type Item = (I::Item, J::Item);

            fn next(&mut self) -> Option<(I::Item, J::Item)> {
                let xi = $x_index_fn(&self.i);
                let x = self.xs.get(xi).unwrap();
                self.i.increment();
                if let Some(ys) = self.x_to_ys.get_mut(&x) {
                    return Some((x, ys.next().unwrap()));
                }
                let mut ys = (self.f)(&x);
                let y = ys.next().unwrap();
                self.x_to_ys.insert(x.clone(), ys);
                Some((x, y))
            }
        }

        pub fn $fn_name<I: Iterator, J: Iterator, F>
            (xs: I,
             f: F)
             -> $struct_name<I, J, F>
            where F: Fn(&I::Item) -> J,
                  I::Item: Clone + Eq + Hash
        {
            $struct_name {
                f: f,
                xs: CachedIterator::new(xs),
                x_to_ys: HashMap::new(),
                i: $index_ctor,
            }
        }
    }
}

exhaustive_dependent_pairs!(ExhaustiveDependentPairsInfiniteLog,
                            exhaustive_dependent_pairs_infinite_log,
                            LogPairIndices,
                            LogPairIndices::new(),
                            |i: &LogPairIndices| i.indices().1);
exhaustive_dependent_pairs!(ExhaustiveDependentPairsInfiniteSqrt,
                            exhaustive_dependent_pairs_infinite_sqrt,
                            SqrtPairIndices,
                            SqrtPairIndices::new(),
                            |i: &SqrtPairIndices| i.y as usize);
exhaustive_dependent_pairs!(ExhaustiveDependentPairsInfinite,
                            exhaustive_dependent_pairs_infinite,
                            ZOrderTupleIndices,
                            ZOrderTupleIndices::new(2),
                            |i: &ZOrderTupleIndices| i.0[1] as usize);
