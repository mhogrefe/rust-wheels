use std::fmt::Display;

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
