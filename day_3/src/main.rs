use std::collections::HashSet;
use std::fs::File;
use std::io::{ BufReader, Result, Lines, prelude::* };
use itertools::Itertools;

fn main() -> Result<()> {
    part_1().and_then(|_| part_2())
}

fn part_1() -> Result<()> {
    let mut sum_of_priorities = 0;
    for line_res in read_file("input.txt") {
        let line = line_res?;
        let half_of_len = line.len() / 2;
        let first_half = &line[0..half_of_len];
        let second_half = &line[half_of_len..line.len()];
        let common = common_character_2(first_half, second_half);
        let priority = char_to_priority(common);
        sum_of_priorities += priority;
    }
    println!("Part one: {}", sum_of_priorities);
    Ok(())
}

fn part_2() -> Result<()> {
    let lines = read_file("input.txt");
    let chunks = &lines.chunks(3);
    let mut sum_of_priorities = 0;

    for chunk in chunks {
        let mut iter = chunk.into_iter();
        let three_lines = [iter.next().unwrap()?, iter.next().unwrap()?, iter.next().unwrap()?];
        let common = common_character_3(three_lines[0].as_str(), three_lines[1].as_str(), three_lines[2].as_str());
        let priority = char_to_priority(common);
        sum_of_priorities += priority;
    }
    println!("Part two: {}", sum_of_priorities);
    Ok(())
}

fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file_name = String::from(file_name);
    let file = File::open(file_name).expect("Can't read a file");
    BufReader::new(file).lines()
}

fn common_character_2(s1: &str, s2: &str) -> char {
    let hs1 = string_to_hashset(s1);
    let hs2 = string_to_hashset(s2);
    *hs1.intersection(&hs2).next().unwrap()
}

fn common_character_3(s1: &str, s2: &str, s3: &str) -> char {
    let hs1 = string_to_hashset(s1);
    let hs2 = string_to_hashset(s2);
    let mut first_intersection: HashSet<char> = HashSet::new();
    for c in hs1.intersection(&hs2) {
        first_intersection.insert(*c);
    }
    let hs3 = string_to_hashset(s3);
    *first_intersection.intersection(&hs3).next().unwrap()
}

fn string_to_hashset(s: &str) -> HashSet<char> {
    let mut res = HashSet::new();
    for c in s.chars() {
        res.insert(c);
    }
    res
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use crate::{char_to_priority, common_character_2, common_character_3};

    #[test]
    fn check_char_to_priority() {
        assert_eq!(1, char_to_priority('a'));
        assert_eq!(2, char_to_priority('b'));
        assert_eq!(26, char_to_priority('z'));
        assert_eq!(27, char_to_priority('A'));
        assert_eq!(52, char_to_priority('Z'));
    }

    #[test]
    fn check_common_character_2() {
        assert_eq!('p', common_character_2("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
        assert_eq!('L', common_character_2("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"));
    }

    #[test]
    fn check_common_character_3() {
        assert_eq!('r', common_character_3("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"));
        assert_eq!('Z', common_character_3("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"));
    }
}