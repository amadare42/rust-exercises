//! Exercise with collections.
//!
//! # Task
//!
//! Given a list of integers, use a vector and return the mean (the average
//! value), median (when sorted, the value in the middle position), and mode
//! (the value that occurs most often; a hash map will be helpful here) of
//! the list.

use std::collections::HashMap;
use std::convert::TryFrom;

fn main() {
    let list = vec![40, 1, 300, 4, 1, 3, 3, 3];
    let result = batch_calc(&list);
    println!("{:?}", result);
}

fn batch_calc(list: &[i32]) -> ListResult {
    let avg = calc_avg(list);
    let median = calc_median(list);
    let mode = calc_mode(list);

    ListResult { avg, median, mode }
}

fn calc_avg(list: &[i32]) -> i32 {
    if !list.is_empty() {
        let sum = list.iter().fold(0, |acc, x| acc + *x);
        match i32::try_from(list.len()) {
            Ok(len) => sum / len,
            _ => 0,
        }
    } else {
        0
    }
}

fn calc_median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    let mut sorted = list.to_owned();
    let idx = list.len() / 2;
    sorted.sort();
    sorted[idx]
}

fn calc_mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for v in list {
        *map.entry(v).or_insert(0) += 1;
    }
    let entry = map.iter().max_by(|(_, x), (_, y)| x.cmp(y));
    match entry {
        Some((key, _)) => **key,
        None => 0,
    }
}

#[derive(Debug)]
struct ListResult {
    avg: i32,
    median: i32,
    mode: i32,
}

pub mod tests;
