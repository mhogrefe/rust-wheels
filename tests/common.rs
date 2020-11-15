extern crate rust_wheels;

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

use rust_wheels::iterators::adaptors::*;

const TINY_LIMIT: usize = 20;
const HUGE_LIMIT: usize = 1_000_000;

#[derive(Debug)]
pub struct TestOutput {
    vecs: HashMap<String, Vec<String>>,
    maps: HashMap<String, Vec<(String, usize)>>,
}

impl TestOutput {
    fn match_vec_helper(&self, key: &str, xs: Vec<String>) {
        let result = self.vecs.get(key);
        if !result.is_some() || &xs != result.unwrap() {
            let mut desired_result_string = String::new();
            desired_result_string.push('\n');
            desired_result_string.push_str(key);
            desired_result_string.push_str(" list ");
            desired_result_string.push_str(&xs.len().to_string());
            desired_result_string.push('\n');
            for x in xs {
                desired_result_string.push_str(&x);
                desired_result_string.push('\n');
            }
            assert!(false, "{}", desired_result_string);
        }
    }

    pub fn match_vec<I>(&self, key: &str, xs: &mut I)
    where
        I: Iterator,
        <I as Iterator>::Item: Display,
    {
        self.match_vec_helper(key, to_limited_string_vec(TINY_LIMIT, xs));
    }

    fn match_vec_f_helper(&self, key: &str, vec: Vec<String>, map: Vec<(String, usize)>) {
        self.match_vec_helper(key, vec);
        let result = self.maps.get(key);
        if !result.is_some() || &map != result.unwrap() {
            let mut desired_result_string = String::new();
            desired_result_string.push('\n');
            desired_result_string.push_str(key);
            desired_result_string.push_str(" map ");
            desired_result_string.push_str(&map.len().to_string());
            desired_result_string.push('\n');
            for (x, f) in map {
                desired_result_string.push_str(&x);
                desired_result_string.push_str(": ");
                desired_result_string.push_str(&f.to_string());
                desired_result_string.push('\n');
            }
            assert!(false, "{}", desired_result_string);
        }
    }

    pub fn match_vec_f<I>(&self, key: &str, xs: &mut I)
    where
        I: Iterator,
        <I as Iterator>::Item: Clone + Display + Eq + Hash,
    {
        let (vec, map) =
            get_limited_string_vec_and_most_common_values(10, TINY_LIMIT, HUGE_LIMIT, xs);
        self.match_vec_f_helper(key, vec, map);
    }
}

pub fn get_expected_test_outputs() -> TestOutput {
    let mut f = File::open("tests/data/test-output.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut vecs = HashMap::new();
    let mut maps = HashMap::new();
    let mut current_key = "";
    let mut current_counter = 0;
    let mut current_vec = Vec::new();
    let mut current_map = Vec::new();
    let mut read_mode = 0;
    for line in contents.lines() {
        match read_mode {
            0 => {
                let tokens = line.split(' ').collect::<Vec<_>>();
                if tokens.len() != 3 {
                    panic!("Bad header: {}", line);
                }
                current_key = tokens[0];
                match tokens[1] {
                    "list" => read_mode = 1,
                    "map" => read_mode = 2,
                    _ => panic!("Bad header: {}", line),
                }
                current_counter = tokens[2].parse().unwrap();
            }
            1 => {
                if current_counter == 0 {
                    if !line.is_empty() {
                        panic!("Line `{}` should be empty", line);
                    }
                    if vecs.contains_key(current_key) {
                        panic!("Duplicate vec key {}", current_key);
                    }
                    vecs.insert(current_key.to_string(), current_vec.clone());
                    current_vec.clear();
                    read_mode = 0;
                } else {
                    current_vec.push(line.to_string());
                    current_counter -= 1;
                }
            }
            2 => {
                if current_counter == 0 {
                    if !line.is_empty() {
                        panic!("Line `{}` should be empty", line);
                    }
                    if maps.contains_key(current_key) {
                        panic!("Duplicate map key {}", current_key);
                    }
                    maps.insert(current_key.to_string(), current_map.clone());
                    current_map.clear();
                    read_mode = 0;
                } else {
                    let line_tokens = line.split(": ").collect::<Vec<_>>();
                    if line_tokens.len() != 2 {
                        panic!("Bad map line: {}", line);
                    }
                    current_map.push((
                        line_tokens[0].to_string(),
                        line_tokens[1].to_string().parse().unwrap(),
                    ));
                    current_counter -= 1;
                }
            }
            _ => panic!("Maps not handled yet"),
        }
    }
    if read_mode == 1 {
        if vecs.contains_key(current_key) {
            panic!("Duplicate key {}", current_key);
        }
        vecs.insert(current_key.to_string(), current_vec.clone());
    } else {
        if maps.contains_key(current_key) {
            panic!("Duplicate map key {}", current_key);
        }
        maps.insert(current_key.to_string(), current_map.clone());
    }
    TestOutput { vecs, maps }
}
