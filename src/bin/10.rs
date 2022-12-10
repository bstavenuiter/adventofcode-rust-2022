use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {

    let mut x = 1;
    let mut cycle = 0;
    let mut value_check_at = 20;
    let mut instr: Vec<&str> = vec![];
    let mut sig_strs: Vec<i32> = vec![];
    let mut current = "";

    for line in input.lines().rev() {
        instr.push(line);
    }

    while instr.len() > 0 {
        cycle = cycle + 1;

        // println!("{cycle} - {current} - {x} ({value_check_at})");
        if cycle == value_check_at {
            sig_strs.push(cycle*x);
            value_check_at += 40;
        }

        if current == "" {
            current = instr.pop().unwrap();
            if current == "noop" {
                current = "";
            }
        } else {
            if current != "noop" {
                x += current.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            }
            current = "";
        }

        if cycle > 1_000_000 {
            panic!("Large loop!");
        }
    }
    let total: i32 = sig_strs.iter().sum();
    println!("{}", total);

    Some(total)
}

pub fn part_two(input: &str) -> Option<String> {

    let mut x = 1;
    let mut cycle = 0;
    let mut instr: Vec<&str> = vec![];
    let mut sig_strs: Vec<i32> = vec![];
    let mut current = "";
    let mut out: String = "".to_string();

    for line in input.lines().rev() {
        instr.push(line);
    }

    while instr.len() > 0 {
        cycle = cycle + 1;

        let mut print_what = ".";
        let post = cycle % 40-1;
        if x - 1 == post || x == post || x+1 == post {
            print_what = "#";
        }
        out += print_what;

        if cycle % 40 == 0{
            out += "\n";
        }

        if current == "" {
            current = instr.pop().unwrap();
            if current == "noop" {
                current = "";
            }
        } else {
            if current != "noop" {
                x += current.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            }
            current = "";
        }

        if cycle > 1_000_000 {
            panic!("Large loop!");
        }
    }

    print!("{out}");
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let input = "noop
addx 3
addx -5";
        assert_eq!(part_one(&input), Some(0));
    }


    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".to_string();
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
