extern crate rust_wheels_lib;

use std::fmt::Display;
use self::rust_wheels_lib::iterators::adaptors::to_limited_string_vec;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const TINY_LIMIT: usize = 20;

#[derive(Debug)]
pub struct TestOutput {
    vecs: HashMap<String, Vec<String>>,
    maps: HashMap<String, HashMap<String, String>>,
}

impl TestOutput {
    pub fn match_list<I>(&self, key: &str, xs: &mut I)
        where I: Iterator,
              <I as Iterator>::Item: Display
    {
        let result = self.vecs.get(key);
        let lsv = to_limited_string_vec(xs, TINY_LIMIT);
        if result.is_some() && &lsv == result.unwrap() {
            assert!(true);
        } else {
            let mut desired_result_string = String::new();
            desired_result_string.push('\n');
            desired_result_string.push_str(key);
            desired_result_string.push_str(" list ");
            desired_result_string.push_str(&lsv.len().to_string());
            desired_result_string.push('\n');
            for x in lsv {
                desired_result_string.push_str(&x);
                desired_result_string.push('\n');
            }
            assert!(false, "{}", desired_result_string);
        }
    }
}

pub fn get_expected_test_outputs() -> TestOutput {
    let mut f = File::open("tests/data/test-output.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let mut vecs: HashMap<String, Vec<String>> = HashMap::new();
    let mut maps: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_key = "";
    let mut current_counter = 0;
    let mut current_vec = Vec::new();
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
                        panic!("Duplicate key {}", current_key);
                    }
                    vecs.insert(current_key.to_string(), current_vec.clone());
                    current_vec.clear();
                    read_mode = 0;
                } else {
                    current_vec.push(line.to_string());
                    current_counter -= 1;
                }
            }
            _ => panic!("Maps not handled yet"),
        }
    }
    if vecs.contains_key(current_key) {
        panic!("Duplicate key {}", current_key);
    }
    vecs.insert(current_key.to_string(), current_vec.clone());
    TestOutput {
        vecs: vecs,
        maps: maps,
    }
}
