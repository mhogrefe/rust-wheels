use iterators::adaptors::Concat;
use iterators::general::CachedIterator;
use iterators::tuples::LogPairIndices;
use std::collections::HashMap;
use std::hash::Hash;

pub fn dependent_pairs<'a, I: Iterator + 'a, J: Iterator, F>
    (xs: I,
     f: &'a F)
     -> Box<Iterator<Item = (I::Item, J::Item)> + 'a>
    where F: Fn(&I::Item) -> J,
          I::Item: Clone
{
    Box::new(Concat::new(xs.map(move |x| f(&x).map(move |y| (x.clone(), y)))))
}

pub struct DependentPairsInfinite<'a, I: Iterator, J: Iterator, F: 'a>
    where I::Item: Clone
{
    f: &'a F,
    xs: CachedIterator<I>,
    x_to_ys: HashMap<I::Item, J>,
    i: LogPairIndices,
}

impl<'a, I: Iterator, J: Iterator, F> Iterator for DependentPairsInfinite<'a, I, J, F>
    where F: Fn(&I::Item) -> J,
          I::Item: Clone + Eq + Hash
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        let (xi, _) = self.i.indices();
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

pub fn dependent_pairs_infinite<'a, I: Iterator + 'a, J: Iterator, F>
    (xs: I,
     f: &'a F)
     -> DependentPairsInfinite<'a, I, J, F>
    where F: Fn(&I::Item) -> J,
          I::Item: Clone + Eq + Hash
{
    DependentPairsInfinite {
        f: f,
        xs: CachedIterator::new(xs),
        x_to_ys: HashMap::new(),
        i: LogPairIndices::new(),
    }
}
