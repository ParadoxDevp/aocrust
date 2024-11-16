use core::num;
use std::{io::BufRead, vec};

use tinyjson::stringify;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut length: usize = 0;

    let numbers: Vec<String> = input.lines().map(|l| {
    length = l.chars().count();
        l.to_string()}).collect();

        let mut gamma = String::from("");
        let mut epsilon = String::from("");

        for n in 0..length {
            let mut zeroes = 0;
            let mut ones = 0;
        

            for number in &numbers {

                if number.chars().nth(n).unwrap() == '0' {
                    zeroes += 1;
                }else {
                    ones += 1;
                }

            }
            if zeroes>ones {                
                gamma.push_str("0");
                epsilon.push_str("1");
            }else {

                gamma.push_str("1");
                epsilon.push_str("0");
            }
        }

        let gamma = u32::from_str_radix(&gamma, 2).unwrap();
        let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

        let power = gamma*epsilon;

        println!("{} {}", gamma, epsilon);
        Some(power)

    
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut length: usize = 0;

    let mut numbers: Vec<String> = input.lines().map(|l| {
    length = l.chars().count();
        l.to_string()}).collect();

        let mut numbers2: Vec<String> = input.lines().map(|l| l.to_string()).collect();


        for n in 0..length {
            let mut zeroes = 0;
            let mut ones = 0;
        

            for number in &numbers {

                if number.chars().nth(n).unwrap() == '0' {
                    zeroes += 1;
                }else {
                    ones += 1;
                }

            }
            if zeroes>ones {                
                numbers.retain(|number| {
                    if number.chars().nth(n).unwrap() == '0' {
                        
                        true 
                    } else {
                        
                        false 
                    }
                });

                numbers2.retain(|number| {
                    if number.chars().nth(n).unwrap() == '1' {
                        
                        true 
                    } else {
                        
                        false 
                    }
                });

                
                
            }else if ones>zeroes{
                numbers.retain(|number| {
                    if number.chars().nth(n).unwrap() == '1' {
                        
                        true 
                    } else {
                        
                        false // This will keep the number in the vector
                    }
                }); 
                numbers2.retain(|number| {
                    if number.chars().nth(n).unwrap() == '0' {
                        
                        true 
                    } else {
                        
                        false 
                    }
                });
                 
            }else if zeroes == ones {
                numbers.retain(|number| {
                    if number.chars().nth(n).unwrap() == '1' {
                        
                        true 
                    } else {
                        
                        false 
                    }
                });
                numbers2.retain(|number| {
                    if number.chars().nth(n).unwrap() == '0' {
                        
                        true 
                    } else {
                        
                        false 
                    }
                });
            }
        }


        for number in &numbers {

           println!("{}", number);

        }
        for number in &numbers2 {

            println!("{}", number);
 
        }

        

       None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        
    }
}