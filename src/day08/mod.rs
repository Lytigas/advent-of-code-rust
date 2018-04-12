#[cfg(test)]
mod solutions {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday08 part1: {}", simulate_registers_max_end(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday08 part2: {}", simulate_registers_max_ever(TEST_CASE));
    }
}

use std::collections::HashMap;

pub fn simulate_registers_max_end(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    for line in input.split('\n') {
        // parse the line
        let mut tokens = line.split(' ');
        let register = tokens.next().unwrap(); // a
        let operation = if tokens.next().unwrap().matches("dec").count() > 0 {
            // dec
            -1
        } else {
            1
        };
        let amount = tokens.next().unwrap().parse::<i32>().unwrap(); // 100
        tokens.next(); // if
        let condo_reg = tokens.next().unwrap(); // b
        let condition = tokens.next().unwrap(); // <
        let value = tokens.next().unwrap().parse::<i32>().unwrap(); // -100
        let current_reg_value = *registers.entry(condo_reg).or_insert(0);
        if match condition {
            "==" => current_reg_value == value,
            "!=" => current_reg_value != value,
            ">" => current_reg_value > value,
            "<" => current_reg_value < value,
            ">=" => current_reg_value >= value,
            "<=" => current_reg_value <= value,
            _ => unreachable!(),
        } {
            *registers.entry(register).or_insert(0) += operation * amount;
        }
    }
    return *registers.iter().map(|x| x.1).max().unwrap();
}

pub fn simulate_registers_max_ever(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut max = ::std::i32::MIN;
    for line in input.split('\n') {
        // parse the line
        let mut tokens = line.split(' ');
        let register = tokens.next().unwrap(); // a
        let operation = if tokens.next().unwrap().matches("dec").count() > 0 {
            // dec
            -1
        } else {
            1
        };
        let amount = tokens.next().unwrap().parse::<i32>().unwrap(); // 100
        tokens.next(); // if
        let condo_reg = tokens.next().unwrap(); // b
        let condition = tokens.next().unwrap(); // <
        let value = tokens.next().unwrap().parse::<i32>().unwrap(); // -100
        let current_reg_value = *registers.entry(condo_reg).or_insert(0);
        if match condition {
            "==" => current_reg_value == value,
            "!=" => current_reg_value != value,
            ">" => current_reg_value > value,
            "<" => current_reg_value < value,
            ">=" => current_reg_value >= value,
            "<=" => current_reg_value <= value,
            _ => unreachable!(),
        } {
            *registers.entry(register).or_insert(0) += operation * amount;
            max = ::std::cmp::max(max, *registers.entry(register).or_insert(0));
        }
    }
    return max;
}
