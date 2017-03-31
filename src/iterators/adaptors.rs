use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub fn to_limited_string_vec<I>(xs: &mut I, size: usize) -> Vec<String>
    where I: Iterator,
          <I as Iterator>::Item: Display
{
    let mut vec = Vec::new();
    let mut found_end = false;
    for _ in 0..size {
        match xs.next() {
            Some(x) => vec.push(x.to_string()),
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

pub fn to_frequency_map<I>(xs: &mut I) -> HashMap<I::Item, u32>
    where I: Iterator,
          <I as Iterator>::Item: Eq + Hash
{
    let mut map: HashMap<I::Item, u32> = HashMap::new();
    for x in xs {
        let frequency = match map.get(&x) {
            None => 0,
            Some(&f) => f,
        };
        map.insert(x, frequency + 1);
    }
    map
}

pub fn get_most_common_values<I>(limit: usize, xs: &mut I) -> Vec<(I::Item, u32)>
    where I: Iterator,
          <I as Iterator>::Item: Clone + Eq + Hash + Display
{
    let map = to_frequency_map(xs);
    let mut inverse_frequency_map: BTreeMap<u32, Vec<I::Item>> = BTreeMap::new();
    for (x, frequency) in map {
        let mut xs = Vec::new();
        xs.push(x);
        match inverse_frequency_map.get(&frequency) {
            Some(ref ys) => xs.extend(ys.iter().cloned()),
            _ => {}
        };
        inverse_frequency_map.insert(frequency, xs);
    }
    let mut most_common_values = Vec::new();
    let mut i = 0;
    for (&frequency, xs) in inverse_frequency_map.iter().rev() {
        let mut sorted_xs = xs.clone();
        sorted_xs.sort_by(|ref a, ref b| a.to_string().cmp(&b.to_string()));
        for x in sorted_xs {
            if i == limit {
                break;
            }
            most_common_values.push((x, frequency));
            i += 1;
        }
        if i == limit {
            break;
        }
    }
    most_common_values
}
