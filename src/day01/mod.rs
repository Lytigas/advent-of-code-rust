#[cfg(test)]
mod solutions {
    #[test]
    use super::*;

    #[test]
    static TEST_CASE: &str = include_str!("input");

    #[test]
    fn part1() {
        println!("\nday01 part1: {}", compute_captcha(TEST_CASE));
    }

    #[test]
    fn part2() {
        println!("\nday01 part2: {}", compute_captcha2(TEST_CASE));
    }
}

pub fn compute_captcha(param: &str) -> u32 {
    let mut sum = 0;
    let vec: Vec<u32> = param
        .chars()
        .map(|x| x.to_digit(10).expect("Must be base 10 numbers"))
        .collect();
    for i in 0..vec.len() {
        if vec.get(i) == vec.get((i + 1) % vec.len()) {
            sum += vec.get(i).unwrap();
        }
    }
    sum
}

pub fn compute_captcha2(param: &str) -> u32 {
    let mut sum = 0;
    let vec: Vec<u32> = param
        .chars()
        .map(|x| x.to_digit(10).expect("Must be base 10 numbers"))
        .collect();
    for i in 0..vec.len() {
        if vec.get(i) == vec.get((i + vec.len() / 2) % vec.len()) {
            sum += vec.get(i).unwrap();
        }
    }
    sum
}
