pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a,b)| get_sum(a, &[b]))
        .sum::<u32>();

    Some(sum)
}

fn get_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid character"),
    }
}

fn get_sum(a: &str, b: &[&str]) -> u32 {
    let mut a_chars = a.chars().collect::<Vec<char>>();
    a_chars.sort();
    a_chars.dedup();
    return a_chars.iter()
        .filter(|a_char| b.iter().all(|r| r.contains(**a_char)))
        .map(|c| get_value(*c))
        .sum::<u32>();
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| get_sum(lines[0], &lines[1..=2]))
        .sum::<u32>();
    Some(sum)
    // None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        // assert_eq!(part_one(&input), None);
        assert_eq!(part_one(&input).unwrap(), 157);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        // assert_eq!(part_two(&input), None);
        assert_eq!(part_two(&input).unwrap(), 70);
    }
}
