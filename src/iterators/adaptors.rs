use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

fn to_limited_string_vec_helper<I>(limit: usize,
                                   xs: &mut I,
                                   f: &Fn(&I::Item) -> String)
                                   -> Vec<String>
    where I: Iterator
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
    if !found_end {
        match xs.next() {
            Some(_) => vec.push("...".to_string()),
            _ => {}
        }
    }
    vec
}

pub fn to_limited_string_vec<I>(limit: usize, xs: &mut I) -> Vec<String>
    where I: Iterator,
          <I as Iterator>::Item: Display
{
    to_limited_string_vec_helper(limit, xs, &|x| x.to_string())
}

pub fn to_limited_string_vec_debug<I>(limit: usize, xs: &mut I) -> Vec<String>
    where I: Iterator,
          <I as Iterator>::Item: Debug
{
    to_limited_string_vec_helper(limit, xs, &|x| format!("{:?}", x))
}

pub fn to_frequency_map<I>(xs: &mut I) -> HashMap<I::Item, usize>
    where I: Iterator,
          <I as Iterator>::Item: Eq + Hash
{
    let mut map: HashMap<I::Item, usize> = HashMap::new();
    for x in xs {
        let frequency = match map.get(&x) {
            None => 0,
            Some(&f) => f,
        };
        map.insert(x, frequency + 1);
    }
    map
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

fn get_most_common_values_helper<T>(limit: usize,
                                    map: HashMap<T, usize>,
                                    f: &Fn(&T) -> String)
                                    -> Vec<(String, usize)>
    where T: Eq + Hash
{
    let mut inverse_frequency_map: HashMap<usize, Vec<T>> = HashMap::new();
    for (x, frequency) in map {
        if inverse_frequency_map.contains_key(&frequency) {
            inverse_frequency_map.get_mut(&frequency).unwrap().push(x);
        } else {
            let mut xs = Vec::new();
            xs.push(x);
            inverse_frequency_map.insert(frequency, xs);
        }
    }
    let mut most_common_values = BinaryHeap::new();
    let mut i = 0;
    for (&frequency, xs) in inverse_frequency_map.iter() {
        for x in xs {
            most_common_values.push(FrequencyRecord {
                                        item: f(x),
                                        frequency: frequency,
                                    });
            if i < limit {
                i += 1;
            } else {
                most_common_values.pop();
            }
        }
    }
    let mut result = Vec::new();
    while let Some(record) = most_common_values.pop() {
        result.push((record.item, record.frequency));
    }
    result.reverse();
    result
}

pub fn get_most_common_values<T>(limit: usize, map: HashMap<T, usize>) -> Vec<(String, usize)>
    where T: Eq + Hash + Display
{
    get_most_common_values_helper(limit, map, &|x| x.to_string())
}

pub fn get_most_common_values_debug<T>(limit: usize, map: HashMap<T, usize>) -> Vec<(String, usize)>
    where T: Eq + Hash + Debug
{
    get_most_common_values_helper(limit, map, &|x| format!("{:?}", x))
}

fn get_limited_string_vec_and_frequency_map_helper<I>(small_limit: usize,
                                                      large_limit: usize,
                                                      xs: &mut I,
                                                      f: &Fn(&I::Item) -> String)
                                                      -> (Vec<String>, HashMap<I::Item, usize>)
    where I: Iterator,
          <I as Iterator>::Item: Eq + Hash
{
    let mut vec = Vec::new();
    let mut map: HashMap<I::Item, usize> = HashMap::new();
    let mut i = 0;
    for x in xs.take(large_limit) {
        if i < small_limit {
            vec.push(f(&x))
        }
        let frequency = match map.get(&x) {
            None => 0,
            Some(&f) => f,
        };
        map.insert(x, frequency + 1);
        i += 1;
    }
    if small_limit < i {
        vec.push("...".to_string());
    }
    (vec, map)
}

pub fn get_limited_string_vec_and_frequency_map<I>(small_limit: usize,
                                                   large_limit: usize,
                                                   xs: &mut I)
                                                   -> (Vec<String>, HashMap<I::Item, usize>)
    where I: Iterator,
          <I as Iterator>::Item: Eq + Hash + Display
{
    get_limited_string_vec_and_frequency_map_helper(small_limit,
                                                    large_limit,
                                                    xs,
                                                    &|x| x.to_string())
}

pub fn get_limited_string_vec_and_frequency_map_debug<I>
    (small_limit: usize,
     large_limit: usize,
     xs: &mut I)
     -> (Vec<String>, HashMap<I::Item, usize>)
    where I: Iterator,
          <I as Iterator>::Item: Eq + Hash + Debug
{
    get_limited_string_vec_and_frequency_map_helper(small_limit,
                                                    large_limit,
                                                    xs,
                                                    &|x| format!("{:?}", x))
}

fn get_limited_string_vec_and_most_common_values_helper<I>(tiny_limit: usize,
                                                           small_limit: usize,
                                                           large_limit: usize,
                                                           xs: &mut I,
                                                           f: &Fn(&I::Item) -> String)
                                                           -> (Vec<String>, Vec<(String, usize)>)
    where I: Iterator,
          <I as Iterator>::Item: Clone + Eq + Hash
{
    let (vec, map) =
        get_limited_string_vec_and_frequency_map_helper(small_limit, large_limit, xs, f);
    (vec, get_most_common_values_helper(tiny_limit, map, f))
}

pub fn get_limited_string_vec_and_most_common_values<I>(tiny_limit: usize,
                                                        small_limit: usize,
                                                        large_limit: usize,
                                                        xs: &mut I)
                                                        -> (Vec<String>, Vec<(String, usize)>)
    where I: Iterator,
          <I as Iterator>::Item: Clone + Eq + Hash + Display
{
    get_limited_string_vec_and_most_common_values_helper(tiny_limit,
                                                         small_limit,
                                                         large_limit,
                                                         xs,
                                                         &|x| x.to_string())
}

pub fn get_limited_string_vec_and_most_common_values_debug<I>
    (tiny_limit: usize,
     small_limit: usize,
     large_limit: usize,
     xs: &mut I)
     -> (Vec<String>, Vec<(String, usize)>)
    where I: Iterator,
          <I as Iterator>::Item: Clone + Eq + Hash + Debug
{
    get_limited_string_vec_and_most_common_values_helper(tiny_limit,
                                                         small_limit,
                                                         large_limit,
                                                         xs,
                                                         &|x| format!("{:?}", x))
}

pub struct MultiChain<I> {
    ranges: Vec<I>,
    i: usize,
}

impl<I> MultiChain<I> {
    pub fn new(ranges: Vec<I>) -> MultiChain<I> {
        MultiChain {
            ranges: ranges,
            i: 0,
        }
    }
}

impl<I> Iterator for MultiChain<I>
    where I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.i == self.ranges.len() {
            None
        } else {
            loop {
                if let Some(x) = self.ranges[self.i].next() {
                    return Some(x);
                } else {
                    self.i += 1;
                    if self.i == self.ranges.len() {
                        return None;
                    }
                }
            }
        }
    }
}
