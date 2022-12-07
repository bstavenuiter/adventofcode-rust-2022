pub fn part_one(input: &str) -> Option<u32> {
    for line in input.lines() {
        println!("{line:?}");
        let start = solve_line(line, 4);
        println!("start at {start:?}");
    }
    None
}

fn solve_line(line: &str, dist_chars: usize) -> usize {
    let mut window = (0, dist_chars);
    let mut start = 0;
    while start == 0 {
        let mut chars: Vec<char> = line[window.0..window.1].chars().collect();
        chars.sort();
        chars.dedup();
        if chars.len() == dist_chars {
            start = window.1;
            break;
        }
        window = (window.0 + 1, window.1 + 1);
        //this is needed for 2
        if window.1 > line.len() {
            break;
        }
    }
    return start;
}

pub fn part_two(input: &str) -> Option<u32> {
    for line in input.lines() {
        println!("{line:?}");
        let start = solve_line(line, 14);
        println!("start at {start:?}");
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
