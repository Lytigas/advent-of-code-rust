#[cfg(test)]
mod solutions {
    #[test]
    use super::*;

    #[test]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday02 part1: {}", checksum(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday02 part2: {}", checksum2(TEST_CASE));
    }
}

pub fn checksum(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|line| -> u32 {
            let l: Vec<u32> = line.split('\t')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            l.iter().max().unwrap() - l.iter().min().unwrap()
        })
        .sum()
}

pub fn checksum2(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|line| -> u32 {
            let l: Vec<u32> = line.split('\t')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            for first in 0..l.len() {
                for second in 0..l.len() {
                    if first == second {
                        continue;
                    }
                    if l[first] % l[second] == 0 {
                        return l[first] / l[second];
                    }
                }
            }
            0
        })
        .sum()
}
