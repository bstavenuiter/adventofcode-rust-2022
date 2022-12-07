pub fn part_one(input: &str) -> Option<u32> {
    let vec = process(input);
    println!("Largest {}", vec[0]);
    return Some(vec[0]);
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = process(input);
    println!("Largest three combined: {}", vec[0] + vec[1] + vec[2]);
    return Some(vec[0] + vec[1] + vec[2]);
}

fn process(input: &str) -> Vec<u32> {
    let mut largest: u32 = 0;
    let mut current: u32 = 0;
    let mut vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line == "" {
            if current > largest {
                largest = current;
            }
            vec.push(current);
            current = 0;
        } else {
            current = current + line.parse::<u32>().unwrap();
        }
    }
    vec.push(current);

    vec.sort_by(|a, b| b.cmp(a));
    return vec;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
