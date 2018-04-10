#[cfg(test)]
mod solutions {
    use super::*;

    static TEST_CASE: &str = include_str!("input");


    #[test]
    fn part1() {
        println!("\nday05 part1: {}", instruction_count(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday05 part2: {}", instruction_count_decrementing(TEST_CASE));
    }
}

pub fn instruction_count(input: &str) -> u32 {
    let mut jumps: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut ptr: i32 = 0;
    let mut ctr = 0;
    loop {
        match jumps.get_mut(ptr as usize) {
            Some(x) => {
                ctr += 1;
                ptr += *x;
                if ptr < 0 {return ctr;}
                *x += 1
            },
            None => {break;}
        }
    }
    return ctr;
}

pub fn instruction_count_decrementing(input: &str) -> u32 {
    let mut jumps: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut ptr: i32 = 0;
    let mut ctr = 0;
    loop {
        match jumps.get_mut(ptr as usize) {
            Some(x) => {
                ctr += 1;
                ptr += *x;
                if ptr < 0 {return ctr;}
                *x = match x {
                    _ if *x >= 3 => *x - 1,
                    _ => *x + 1
                };
            },
            None => {break;}
        }
    }
    return ctr;
}