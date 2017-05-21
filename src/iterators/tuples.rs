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

pub struct ExhaustivePairsFromSingle<I: Iterator>
    where I::Item: Clone
{
    xs: CachedIterator<I>,
    i: ZOrderTupleIndices,
    stop_checking_size: bool,
    max_indices: Option<ZOrderTupleIndices>,
}

impl<I: Iterator> Iterator for ExhaustivePairsFromSingle<I>
    where I::Item: Clone
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let i = self.i.0[0] as usize;
            let j = self.i.0[1] as usize;
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
                    let size = size as u64;
                    self.max_indices = Some(ZOrderTupleIndices(vec![size - 1, size - 1]));
                    self.stop_checking_size = true;
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn exhaustive_pairs_from_single<I: Iterator>(xs: I) -> ExhaustivePairsFromSingle<I>
    where I::Item: Clone
{
    ExhaustivePairsFromSingle {
        xs: CachedIterator::new(xs),
        i: ZOrderTupleIndices::new(2),
        stop_checking_size: false,
        max_indices: None,
    }
}

pub struct ExhaustivePairs<I: Iterator, J: Iterator>
    where I::Item: Clone,
          J::Item: Clone
{
    xs: CachedIterator<I>,
    ys: CachedIterator<J>,
    i: ZOrderTupleIndices,
    stop_checking_size: bool,
    max_indices: Option<ZOrderTupleIndices>,
}

impl<I: Iterator, J: Iterator> Iterator for ExhaustivePairs<I, J>
    where I::Item: Clone,
          J::Item: Clone
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let i = self.i.0[0] as usize;
            let j = self.i.0[1] as usize;
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
                        self.max_indices = Some(ZOrderTupleIndices(vec![xs_size as u64 - 1,
                                                                        ys_size as u64 - 1]));
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
pub fn exhaustive_pairs<I: Iterator, J: Iterator>(xs: I, ys: J) -> ExhaustivePairs<I, J>
    where I::Item: Clone,
          J::Item: Clone
{
    ExhaustivePairs {
        xs: CachedIterator::new(xs),
        ys: CachedIterator::new(ys),
        i: ZOrderTupleIndices::new(2),
        stop_checking_size: false,
        max_indices: None,
    }
}

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
