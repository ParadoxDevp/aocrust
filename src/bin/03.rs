use core::num;
use std::{io::BufRead, vec};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    
    let numbers: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    let length = 12;
    // let length = numbers[0].count_ones() + numbers[0].count_zeros();
    println!("{}", 12);

    let mut num = vec![0; length.try_into().unwrap()];

        for n in 0..length{
            let mut zeroes: u32 = 0;
            let mut ones: u32 = 0;

            for number in &numbers {

                if number >> n&1 == 0{
                    zeroes+=1;
                }else {
                    ones+=1;
                }

                if zeroes > ones {
                    num[<u32 as TryInto<usize>>::try_into(n).unwrap()] = 0;  
                }else {
                    num[<u32 as TryInto<usize>>::try_into(n).unwrap()] = 1;  
                }


            }

        }
        let gammabin = num.iter().map(|digit| digit.to_string()).collect::<String>();
        let gammabin = &gammabin;
        let gamma = u32::from_str_radix(gammabin, 2).unwrap();
        let epsilonbin = gammabin.chars()
        .map(|ch| if ch == '1' { '0' } else { '1' })
        .collect::<String>();
        let epsilonbin = &epsilonbin;
        let epsilon = u32::from_str_radix(epsilonbin, 2).unwrap();

        let power = gamma*epsilon;

    Some(power)
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
