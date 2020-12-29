use iterators::general::{random, Random};
use iterators::vecs::{random_vecs, RandomVecs};

pub struct RandomStrings<I>(RandomVecs<I>)
where
    I: Iterator<Item = char>;

impl<I> Iterator for RandomStrings<I>
where
    I: Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.0.next().map(|s| s.into_iter().collect())
    }
}

pub fn random_strings_with_chars<I>(
    seed: &[u32],
    scale: u32,
    chars_gen: &dyn Fn(&[u32]) -> I,
) -> RandomStrings<I>
where
    I: Iterator<Item = char>,
{
    RandomStrings(random_vecs(seed, scale, chars_gen))
}

pub fn random_strings(seed: &[u32], scale: u32) -> RandomStrings<Random<char>> {
    random_strings_with_chars(seed, scale, &|seed_2| random(seed_2))
}
