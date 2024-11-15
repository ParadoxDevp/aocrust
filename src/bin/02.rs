advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result: Vec<(&str, u32)> = input.lines().map(|l| {
        let l = l.split_once(' ').unwrap();
        let direction = l.0;
        let amount = l.1.parse().unwrap();
        (direction, amount)

    }).collect();


    let mut depth = 0;
    let mut forward = 0;

    for res in result {
        match res.0 {
            "forward" => forward += res.1,
            "down" => depth += res.1,
            "up" => depth -= res.1,
            _ => depth = depth,
        }
    }

    Some(depth * forward)
}

pub fn part_two(input: &str) -> Option<u32> {

    let result: Vec<(&str, u32)> = input.lines().map(|l| {
        let l = l.split_once(' ').unwrap();
        let direction = l.0;
        let amount = l.1.parse().unwrap();
        (direction, amount)

    }).collect();

    let mut depth = 0;
    let mut aim = 0;
    let mut forward = 0;

    for res in result {
        match res.0 {
            "forward" => {
                forward += res.1;
                depth += aim * res.1;
            }
            "down" => aim += res.1,
            "up" => aim -= res.1,
            _ => depth = depth,
        }
    }

    Some(depth * forward)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
