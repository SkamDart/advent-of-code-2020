use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> io::Result<u32> {
    let lines = read_lines("day2.txt")?;
    Ok(lines.into_iter().map(|line| is_valid_password(line.unwrap()) as u32).sum::<u32>())
}

fn part2() -> io::Result<u32> {
    let lines = read_lines("day2.txt")?;
    Ok(lines.into_iter().map(|line| is_correct_password(line.unwrap()) as u32).sum::<u32>())
}

fn is_correct_password(line: String) -> bool {
    match line.split(" ").collect::<Vec<&str>>().as_slice() {
        [range, ch, password] => {
            let ch = ch.chars().nth(0).unwrap();
            match range.split("-").collect::<Vec<&str>>().as_slice() {
                [min, max] => {
                    let min_idx = min.parse::<usize>().unwrap() - 1;
                    let max_idx = max.parse::<usize>().unwrap() - 1;
                    (ch == password.chars().nth(min_idx).unwrap()) ^ (ch == password.chars().nth(max_idx).unwrap())
                }
                _ => false,
            }
        }
        _ => false
    }
}

fn is_valid_password(line: String) -> bool {
    match line.split(" ").collect::<Vec<&str>>().as_slice() {
        [range, ch, password] => {
            let ch = ch.chars().nth(0).unwrap();
            let frequency = password.matches(ch).into_iter().count() as u32;
            match range.split("-").collect::<Vec<&str>>().as_slice() {
                [min, max] => min.parse::<u32>().unwrap() <= frequency && frequency <= max.parse::<u32>().unwrap(),
                _ => false,
            }
        }
        _ => false
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
