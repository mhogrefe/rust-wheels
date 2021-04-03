use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct FrequencyRecord {
    item: String,
    frequency: usize,
}

impl Ord for FrequencyRecord {
    fn cmp(&self, other: &FrequencyRecord) -> Ordering {
        match other.frequency.cmp(&self.frequency) {
            Ordering::Equal => self.item.cmp(&other.item),
            c => c,
        }
    }
}

impl PartialOrd for FrequencyRecord {
    fn partial_cmp(&self, other: &FrequencyRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) struct Concat<I: Iterator> {
    xss: I,
    xs: Option<I::Item>,
}

impl<I: Iterator> Concat<I> {
    pub(crate) fn new(xss: I) -> Concat<I> {
        Concat { xss, xs: None }
    }
}

impl<I: Iterator> Iterator for Concat<I>
where
    I::Item: Iterator,
{
    type Item = <<I as Iterator>::Item as Iterator>::Item;

    fn next(&mut self) -> Option<<<I as Iterator>::Item as Iterator>::Item> {
        if self.xs.is_none() {
            match self.xss.next() {
                None => return None,
                Some(xs) => self.xs = Some(xs),
            }
        }
        loop {
            match self.xs.as_mut().unwrap().next() {
                None => match self.xss.next() {
                    None => return None,
                    Some(xs) => self.xs = Some(xs),
                },
                Some(x) => return Some(x),
            }
        }
    }
}
