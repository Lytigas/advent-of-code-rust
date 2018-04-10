#[cfg(test)]
mod solutions {
    use super::*;

    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday04 part1: {}", number_valid(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday04 part2: {}", number_valid_strict(TEST_CASE));
    }
}

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hasher;
use std::hash::Hash;
use std::cmp::Ordering;

pub fn number_valid(input: &str) -> usize {
    input.split('\n').filter(|line| {
        let mut u: HashSet<&str> = HashSet::new();
        for word in line.split_whitespace() {
            if u.contains(word) {
                return false;
            }
            u.insert(word);
        }
        true
    }).count()
}

#[derive(PartialEq, Eq)]
struct Wrapper {
    map: HashMap<char, u32>
}

impl Wrapper {
    pub fn entry(&mut self, key: char) -> Entry<char, u32> {
        self.map.entry(key)
    }
}

impl Hash for Wrapper {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut v:Vec<(char, u32)> = self.map.iter().map(|x| (*x.0, *x.1)).collect();
        v.sort_unstable_by(|x, y| {
            if x.0 > y.0 {
                Ordering::Greater
            } else if x.0 < y.0 {
                Ordering::Less
            } else {
                if x.1 > y.1 {
                    Ordering::Greater
                } else if x.1 < y.1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });
        v.hash(state)
    }
}

pub fn number_valid_strict(input: &str) -> usize {
    input.split('\n').filter(|line| {

        let mut u: HashSet<Wrapper> = HashSet::new();

        for word in line.split_whitespace() {
            let mut c = Wrapper {map: HashMap::new()};
            for char in word.chars() {
                *c.entry(char).or_insert(0) += 1
            }

            if u.contains(&c) {
                return false;
            }
            u.insert(c);
        }
        true
    }).count()
}