#[cfg(test)]
mod solutions {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday06 part1: {}", num_cycles(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday06 part2: {}", num_cycles_loop(TEST_CASE));
    }
}

use std::collections::HashSet;

pub fn num_cycles(input: &str) -> u32 {
    let mut memory: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut states: HashSet<Vec<u32>> = HashSet::new();
    let mut ctr = 0;

    while !states.contains(&memory) {
        states.insert(memory.clone());
        if let Some((index, &val)) = memory.iter().enumerate().rev().max_by_key(|x| x.1) {
            memory[index] = 0;
            let len = memory.len();
            for i in 0..len {
                *memory.get_mut(i).unwrap() += val / len as u32;
            }
            for i in (index + 1)..(index + 1 + (val % len as u32) as usize) {
                *memory.get_mut(i % len).unwrap() += 1;
            }
        }
        ctr += 1;
    }
    ctr
}

use std::collections::HashMap;

pub fn num_cycles_loop(input: &str) -> u32 {
    let mut memory: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut states: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut ctr = 0;

    while !states.contains_key(&memory) {
        states.insert(memory.clone(), ctr);
        if let Some((index, &val)) = memory.iter().enumerate().rev().max_by_key(|x| x.1) {
            memory[index] = 0;
            let len = memory.len();
            for i in 0..len {
                *memory.get_mut(i).unwrap() += val / len as u32;
            }
            for i in (index + 1)..(index + 1 + (val % len as u32) as usize) {
                *memory.get_mut(i % len).unwrap() += 1;
            }
        }
        ctr += 1;
    }
    ctr - states.get(&memory).unwrap()
}
