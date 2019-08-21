use iterators::adaptors::MultiChain;
use iterators::chars::exhaustive_chars;
use iterators::general::{random, Random, RangeIncreasing};
use iterators::vecs::{exhaustive_vecs, random_vecs, ExhaustiveVecs, RandomVecs};

pub struct ExhaustiveStrings<'a, I>(ExhaustiveVecs<'a, I>)
where
    I: Clone + Iterator<Item = char>;

impl<'a, I> Iterator for ExhaustiveStrings<'a, I>
where
    I: Clone + Iterator<Item = char>,
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.0.next().map(|s| s.into_iter().collect())
    }
}

pub fn exhaustive_strings_with_chars<'a, I>(chars: I) -> ExhaustiveStrings<'a, I>
where
    I: 'a + Clone + Iterator<Item = char>,
{
    ExhaustiveStrings(exhaustive_vecs(chars))
}

pub fn exhaustive_strings<'a>() -> ExhaustiveStrings<'a, MultiChain<RangeIncreasing<char>>> {
    exhaustive_strings_with_chars(exhaustive_chars())
}

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
