#[cfg(test)]
mod solutions {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday09 part1: {}", total_score(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday09 part2: {}", count_garbage_chars(TEST_CASE));
    }
}

pub fn total_score(input: &str) -> u32 {
    let tokens: Vec<char> = input.chars().collect(); // can't assume ascii :)
    let mut cleanup_parsed: Vec<char> = Vec::with_capacity(tokens.len());
    let mut in_garbage = false;
    let mut index = 0;
    while index < tokens.len() {
        if in_garbage {
            match tokens[index] {
                '!' => index += 2,
                '>' => {
                    in_garbage = false;
                    index += 1;
                }
                _ => index += 1,
            }
            continue;
        }
        if tokens[index] == '<' {
            in_garbage = true;
            index += 1;
            continue;
        }
        cleanup_parsed.push(tokens[index]);
        index += 1;
    }
    // println!("{}", cleanup_parsed.iter().cloned().collect::<String>());
    let mut group_level = 0;
    let mut total = 0;
    for token in cleanup_parsed.iter() {
        match *token {
            '{' => {
                group_level += 1;
            }
            '}' => {
                total += group_level;
                group_level -= 1;
            }
            _ => (),
        }
    }
    return total;
}

pub fn count_garbage_chars(input: &str) -> u32 {
    let tokens: Vec<char> = input.chars().collect(); // can't assume ascii :)
    let mut in_garbage = false;
    let mut index = 0;
    let mut total = 0;
    while index < tokens.len() {
        if in_garbage {
            match tokens[index] {
                '!' => index += 2,
                '>' => {
                    in_garbage = false;
                    index += 1;
                }
                _ => {
                    index += 1;
                    total += 1;
                }
            }
            continue;
        }
        if tokens[index] == '<' {
            in_garbage = true;
            index += 1;
            continue;
        }
    }
    return total;
}
