use iterators::common::scramble;
use rand::{IsaacRng, Rng, SeedableRng};
use std::iter::{Chain, Once, once};

//TODO test
pub fn exhaustive_with_element<I>(x: I::Item, xs: I) -> Chain<Once<I::Item>, I>
    where I: Iterator
{
    once(x).chain(xs)
}

pub struct RandomWithElement<I>
    where I: Iterator
{
    rng: IsaacRng,
    weight: u32,
    x: I::Item,
    xs: I,
}

impl<I> Iterator for RandomWithElement<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.rng.gen_weighted_bool(self.weight) {
            Some(self.x.clone())
        } else {
            self.xs.next()
        }
    }
}

//TODO test
pub fn random_with_element<I>(seed: &[u32],
                              scale: u32,
                              x: I::Item,
                              xs_gen: &Fn(&[u32]) -> I)
                              -> RandomWithElement<I>
    where I: Iterator
{
    RandomWithElement {
        rng: IsaacRng::from_seed(&scramble(seed, "x")),
        weight: scale + 2,
        x,
        xs: xs_gen(&scramble(seed, "xs")),
    }
}

pub struct Somes<I>(I) where I: Iterator;

impl<I: Iterator> Iterator for Somes<I> {
    type Item = Option<I::Item>;

    fn next(&mut self) -> Option<Option<I::Item>> {
        self.0.next().map(Option::from)
    }
}

//TODO test
pub fn exhaustive_options<I: Iterator>(xs: I) -> Chain<Once<Option<I::Item>>, Somes<I>> {
    once(Option::None).chain(Somes(xs))
}

pub struct RandomOptions<I: Iterator> {
    rng: IsaacRng,
    weight: u32,
    xs: I,
}

impl<I: Iterator> Iterator for RandomOptions<I> {
    type Item = Option<I::Item>;

    fn next(&mut self) -> Option<Option<I::Item>> {
        if self.rng.gen_weighted_bool(self.weight) {
            Some(None)
        } else {
            Some(self.xs.next())
        }
    }
}

//TODO test
pub fn random_options<I: Iterator>(seed: &[u32],
                                   scale: u32,
                                   xs_gen: &Fn(&[u32]) -> I)
                                   -> RandomOptions<I> {
    RandomOptions {
        weight: scale + 2,
        rng: IsaacRng::from_seed(&scramble(seed, "none")),
        xs: xs_gen(&scramble(seed, "some")),
    }
}
