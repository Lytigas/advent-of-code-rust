#[cfg(test)]
mod solutions {
    #[test]
    use super::*;

    #[test]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday03 part1: {}", distance2(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday03 part2: {}", first_allocate(TEST_CASE));
    }
}

use std::cmp;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use self::Direction::*;

struct Step {
    step: u32,
    direction: Direction,
    do_increment: bool,
}

impl Step {
    fn mutate(&mut self) {
        self.step = self.step + (if self.do_increment { 1 } else { 0 });
        self.direction = match self.direction {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        };
        self.do_increment = !self.do_increment;
    }
}

pub fn distance2(input: &str) -> u32 {
    let address = input.parse::<i32>().unwrap();
    let mut current: (i32, i32) = (0, 0);
    let mut address_so_far = 1;
    let mut step = Step {
        step: 1,
        direction: Right,
        do_increment: false,
    };

    loop {
        // do step
        let step_this_time = cmp::min(step.step as i32, address - address_so_far);
        match step.direction {
            Up => current.1 += step_this_time,
            Left => current.0 -= step_this_time,
            Down => current.1 -= step_this_time,
            Right => current.0 += step_this_time,
        }
        address_so_far += step.step as i32;
        if address_so_far > address {
            return (current.0.abs() + current.1.abs()) as u32;
        }
        step.mutate();
    }
}

use std::collections::HashMap;

pub fn first_allocate(input: &str) -> u32 {
    let value = input.parse::<u32>().unwrap();
    // x, y, increment_steps
    let step_types = [(1, 0, 1), (0, 1, 0), (-1, 0, 1), (0, -1, 0)];
    let mut memory: HashMap<(i32, i32), u32> = HashMap::new();
    let mut current = (0, 0);
    let mut step = 0;

    memory.insert(current, 1);

    loop {
        for step_type in step_types.iter() {
            step += step_type.2;
            for _ in 0..step {
                current.0 += step_type.0;
                current.1 += step_type.1;
                let mut sum = 0;
                for x in current.0 - 1..current.0 + 2 {
                    for y in current.1 - 1..current.1 + 2 {
                        sum += match memory.get(&(x, y)) {
                            Some(x) => *x,
                            None => 0,
                        }
                    }
                }
                if sum > value {
                    return sum;
                }
                memory.insert(current, sum);
            }
        }
    }
}
