use std::{result, vec};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut count = 0;
    let numbers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    for i in 0..numbers.len()-1 {
        let a = numbers[i];
        let b = numbers[i+1];
        if b>a {
            count+=1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut numbers: Vec<i32> = vec![];

    let mut threesum: Vec<i32> = vec![];

    
    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }

    for i in 0..numbers.len()-2 {
        let a = numbers[i];
        let b = numbers[i+1];
        let c = numbers[i+2];

        threesum.push(a+b+c);

        }

        let result = threesum.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("\n");
        let results = result.as_str();
        return part_one(results);
    
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
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
