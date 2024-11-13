use std::{result, vec};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let numbers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let mut a = 0;

    for b in numbers {
        if b > a && a != 0 {
            count += 1;
        }
        a = b;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    let numbers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let threesum: Vec<u32> = numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    let mut a = 0;

    for b in threesum {
        if b > a && a != 0 {
            count += 1;
        }
        a = b;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
