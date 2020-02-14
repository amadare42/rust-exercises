//! Exercise of pig 🐽 latin conversion.
//!
//! # Task
//!
//! Convert strings to pig latin. The first consonant of each word is moved to
//! the end of the word and `ay` is added, so `first` becomes `irst-fay`.
//! Words that start with a vowel have `hay` added to the end instead (`apple`
//! becomes `apple-hay`).
//!
//! Keep in mind the details about UTF-8 encoding!

use std::iter::*;
use std::str::Chars;

fn main() {
    to_pig_latin(&String::from("What should we   do with the drunken sailor? Oh well, nothing I guess. "));
    to_pig_latin(&String::from(" What should we do with the drunken sailor? Oh well, nothing I guess. u o"));
    to_pig_latin(&String::from(" What should we do with the drunken sailor? Oh well, nothing I guess.  "));
    to_pig_latin(&String::from(" What should we do   with the drunken sailor? Oh well, nothing I guess.  "));
}

struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn has_chars(&self) -> bool {
        self.to > self.from
    }
    fn move_to(&mut self, idx: usize) {
        self.to = idx;
        self.from = idx;
    }
    fn extend(&mut self) {
        self.to += 1
    }
}

fn to_pig_latin(s: &str) {
    let mut result = String::with_capacity(s.len());

    let mut range = Range { from: 0, to: 0 };

    for (idx, ch) in s.char_indices() {
        match ch {
            _ if ch.is_alphabetic() => {
                if !range.has_chars() {
                    range.move_to(idx);
                }
                range.extend();
            }
            _ => {
                if range.has_chars() {
                    append_range(s, &mut result, &range);
                }
                result.push(ch);
                range.move_to(idx);
            }
        }
    }

    if range.has_chars() {
        append_range(s, &mut result, &range);
    }

    println!("{} -> {}", s, result);
}

fn append_range(source: &str, target: &mut String, range: &Range) {
    let word = &source[range.from..range.to];
    let word = word_to_pig_latin(word);
    target.push_str(&word);
}

fn word_to_pig_latin(source: &str) -> String {
    let mut chars = source.chars().peekable();
    match chars.peek() {
        None => {
            String::from(source)
        }

        Some(ch) => {
            match ch.to_lowercase().nth(0).unwrap() {
                // vowel
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    let mut s = String::from_iter(chars);
                    s.push_str("-hay");
                    s
                }

                //consonant
                _ => {
                    handle_consonant(&mut chars)
                }
            }
        }
    }
}

fn handle_consonant(chars: &mut Peekable<Chars>) -> String {
    let first_char = chars.next()
        .expect("iterator with at least one character");

    if first_char.is_uppercase() {
        match chars.next() {
            Some(ch) => {
                return String::from(format!("{}{}-{}ay", ch.to_uppercase(), String::from_iter(chars), first_char));
            }
            None => {
                String::from(format!("{}-{}ay", String::from_iter(chars), first_char))
            }
        }
    } else {
        String::from(format!("{}-{}ay", String::from_iter(chars), first_char))
    }
}
