use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"^mask = (?P<mask>[X10]+)$").unwrap();
    static ref MEM_REGEX: Regex =
        Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
}

fn main() {
    let part_one = solve_part_1(include_str!("../input.txt").lines());
    println!("part one: {}", part_one);

    let part_two = solve_part_2(include_str!("../input.txt").lines());
    println!("part two: {}", part_two);
}

fn solve_part_1<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    let mut true_mask: u64 = 0;
    let mut false_mask: u64 = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        if let Some(mask_caps) = MASK_REGEX.captures(line) {
            true_mask = 0;
            false_mask = 0;
            for (idx, char) in mask_caps["mask"].chars().enumerate() {
                match char {
                    '1' => true_mask |= 1 << (35 - idx),
                    '0' => false_mask |= 1 << (35 - idx),
                    _ => (),
                }
            }
        } else if let Some(mem_caps) = MEM_REGEX.captures(line) {
            let address = mem_caps["address"].parse::<u64>().unwrap();
            let mut value = mem_caps["value"].parse::<u64>().unwrap();
            value |= true_mask;
            value &= !false_mask;
            memory.insert(address, value);
        } else {
            panic!("bad line: {}", line);
        }
    }
    memory.values().sum()
}

fn solve_part_2<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    let mut true_mask: u64 = 0;
    let mut floating_masks: Vec<u64> = Vec::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        if let Some(mask_caps) = MASK_REGEX.captures(line) {
            true_mask = 0;
            floating_masks = Vec::new();
            for (idx, char) in mask_caps["mask"].chars().enumerate() {
                match char {
                    '1' => true_mask |= 1 << (35 - idx),
                    'X' => floating_masks.push(35 - idx as u64),
                    _ => (),
                }
            }
        } else if let Some(mem_caps) = MEM_REGEX.captures(line) {
            let mut base_address = mem_caps["address"].parse::<u64>().unwrap();
            let value = mem_caps["value"].parse::<u64>().unwrap();
            base_address |= true_mask;
            let mut masks: Vec<(u64, u64)> = Vec::new();
            for i in 0..(2u64.pow(floating_masks.len() as u32)) {
                let mut true_mask: u64 = 0;
                let mut false_mask: u64 = 0;
                for (j, m) in floating_masks.iter().enumerate() {
                    if i & (1u64 << j as u64) != 0 {
                        true_mask |= 1u64 << m;
                    } else {
                        false_mask |= 1u64 << m;
                    }
                }
                masks.push((true_mask, false_mask));
            }
            for (t, f) in masks {
                let address = (base_address | t) & (!f);
                memory.insert(address, value);
            }
        } else {
            panic!("bad line: {}", line);
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    const INPUT_2: &str = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
    #[test]
    fn it_calculates_the_correct_value_for_part_1() {
        let result = solve_part_1(INPUT_1.lines());
        assert_eq!(165, result);
    }

    #[test]
    fn it_calculates_the_correct_value_for_part_2() {
        let result = solve_part_2(INPUT_2.lines());
        assert_eq!(208, result);
    }
}
