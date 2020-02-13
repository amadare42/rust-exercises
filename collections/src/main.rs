//! Exercise with collections.
//!
//! # Task
//!
//! Given a list of integers, use a vector and return the mean (the average
//! value), median (when sorted, the value in the middle position), and mode
//! (the value that occurs most often; a hash map will be helpful here) of
//! the list.

use std::collections::HashMap;

fn main() {
    let list = vec![40, 1, 300, 4, 1, 3, 3, 3];
    let result = list_med(&list);
    println!("{:?}", result);
}

fn list_med(list: &[i32]) -> ListResult {
    let len = list.len();

    let avg = {
        let mut sum = 0;
        for v in list {
            sum += *v;
        }
        sum / len as i32 // TODO
    };

    let median = {
        let mut sorted = list.to_owned();
        let idx = len / 2;
        sorted.sort();
        sorted[idx]
    };

    let mode = {
        let mut map = HashMap::new();
        for v in list {
            *map.entry(v).or_insert(0) += 1; // NICE
        }
        let mut max: Option<(&i32, &i32)> = None;
        for (key, val) in map.iter() {
            match max {
                Some((_, cval)) => {
                    if *val > *cval {
                        max = Some((key, val))
                    }
                }
                None => max = Some((key, val)) // FIXME
            }
        }
        match max {
            Some((key, _)) => *key,
            None => 0 // FIXME
        }
    };

    ListResult { avg, median, mode }
}

#[derive(Debug)]
struct ListResult { avg: i32, median: i32, mode: i32 }

// TODO: tests
