use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::{Binary, Display};

use malachite_base::strings::ToDebugString;

pub struct GeneratorFromFunction<F, T>(F)
where
    F: FnMut() -> T;

pub fn generate_from_function<F, T>(f: F) -> GeneratorFromFunction<F, T>
where
    F: FnMut() -> T,
{
    GeneratorFromFunction(f)
}

impl<F, T> Iterator for GeneratorFromFunction<F, T>
where
    F: FnMut() -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.0())
    }
}

fn to_limited_string_vec_helper<I>(
    limit: usize,
    xs: &mut I,
    f: &dyn Fn(&I::Item) -> String,
) -> Vec<String>
where
    I: Iterator,
{
    let mut vec = Vec::new();
    let mut found_end = false;
    for _ in 0..limit {
        match xs.next() {
            Some(x) => vec.push(f(&x)),
            None => {
                found_end = true;
                break;
            }
        }
    }
    if !found_end && xs.next().is_some() {
        vec.push("...".to_string())
    }
    vec
}

pub fn to_limited_string_vec<I>(limit: usize, xs: &mut I) -> Vec<String>
where
    I: Iterator,
    <I as Iterator>::Item: Display,
{
    to_limited_string_vec_helper(limit, xs, &|x| x.to_string())
}

fn to_limited_string_helper<I>(limit: usize, xs: &mut I, f: &dyn Fn(&I::Item) -> String) -> String
where
    I: Iterator,
{
    let mut string = String::new();
    let mut found_end = false;
    let mut delimiter = "";
    for _ in 0..limit {
        match xs.next() {
            Some(x) => {
                string.push_str(delimiter);
                delimiter = ", ";
                string.push_str(&f(&x))
            }
            None => {
                found_end = true;
                break;
            }
        }
    }
    if !found_end && xs.next().is_some() {
        string.push_str(delimiter);
        string.push_str("...")
    }
    string
}

pub fn to_limited_string<I>(limit: usize, xs: &mut I) -> String
where
    I: Iterator,
    <I as Iterator>::Item: Display,
{
    to_limited_string_helper(limit, xs, &|x| x.to_string())
}

pub fn to_limited_string_debug<I>(limit: usize, xs: &mut I) -> String
where
    I: Iterator,
    <I as Iterator>::Item: Debug,
{
    to_limited_string_helper(limit, xs, &ToDebugString::to_debug_string)
}

pub fn to_limited_string_binary<I>(limit: usize, xs: &mut I) -> String
where
    I: Iterator,
    <I as Iterator>::Item: Binary,
{
    to_limited_string_helper(limit, xs, &|x| format!("{:#b}", x))
}

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
