pub fn part_one(input: &str) -> Option<u32> {
    let mut amount_overlap = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();
        let (left_start, left_end) = left.split_once("-").map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unwrap();
        let (right_start, right_end) = right.split_once("-").map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unwrap();

        if right_start <= left_start && right_end >= left_end {
            amount_overlap = amount_overlap + 1;
        } else if left_start <= right_start && left_end >= right_end {
            amount_overlap = amount_overlap + 1;
        }
    }
    return Some(amount_overlap);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut amount_overlap = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();
        // let (left_start, left_end) = left.split_once('-').unwrap();
        let (left_start, left_end) = left.split_once("-").map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unwrap();
        let (right_start, right_end) = right.split_once("-").map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unwrap();

        if left_start >= right_start && left_start <= right_end {
            amount_overlap = amount_overlap + 1;
        } else if right_start >= left_start && right_start <= left_end {
            amount_overlap = amount_overlap + 1;
        }
    }
    return Some(amount_overlap);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        // assert_eq!(part_one(&input), None);
        assert_eq!(part_one(&input).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        // assert_eq!(part_two(&input), None);
        assert_eq!(part_two(&input).unwrap(), 4);
    }
}
