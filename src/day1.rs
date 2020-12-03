use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}

fn part_two() -> Option<u32> {
    if let Ok(lines) = read_lines("day1.txt") {
        let mut nums = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let expense = line.parse::<u32>().unwrap();
            nums.push(expense);
        }
        let size = nums.len();
        for i in (std::ops::Range { start: 0, end: size - 2}) {
            for j in (std::ops::Range { start: i + 1, end: size - 1}) {
                for k in (std::ops::Range { start: j + 1, end: size}) {
                    if (nums[i] + nums[j] + nums[k]) == 2020 {
                        return Some(nums[i] * nums[j] * nums[k]);
                    }
                }
            }
        }
    }
        None
}

fn part_one() -> Option<u32> {
    if let Ok(lines) = read_lines("day1.txt") {
        let mut want_to_have: HashMap<u32, u32> = HashMap::new();
        for line in lines {
            let line = line.unwrap();
            let expense = line.parse::<u32>().unwrap();
            let want = 2020 - expense;
            if !want_to_have.contains_key(&expense) {
                want_to_have.insert(want, expense);
            } else {
                println!("{} x {} = {}", expense, want, expense * want);
                return Some(expense * want)
            }
        }
        None
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
