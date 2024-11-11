advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<i32> = vec![];
    let mut count = 0;

    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }

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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
