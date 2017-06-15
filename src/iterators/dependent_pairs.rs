use iterators::adaptors::Concat;

pub fn exhaustive_dependent_pairs<'a, I: Iterator + 'a, J: Iterator, F>
    (xs: I,
     f: &'a F)
     -> Box<Iterator<Item = (I::Item, J::Item)> + 'a>
    where F: Fn(&I::Item) -> J,
          I::Item: Clone
{
    Box::new(Concat::new(xs.map(move |x| f(&x).map(move |y| (x.clone(), y)))))
}
