
pub fn part_one(input: &str) -> Option<u32> {

    let report_amount = 100_000;
    let total_space = 70_000_000;
    let space_to_delete = 30_000_000;

    let mut final_countdown = vec![];
    let mut stack = vec![("/", 0)];
    let mut part_2_total = 0;
    let mut part_1_total = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = &line[5..];
            if dir == ".." { 
                let (name, amount) = stack.pop().unwrap();
                if amount <= report_amount {
                    part_1_total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
            } else {
                stack.push((dir, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        } else if amount == "dir" {
            //ignore
        }
    }

    while stack.len() > 0{
        let (name, amount) = stack.pop().unwrap();
        final_countdown.push((name, amount));
        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }
    let free_space = total_space - final_countdown.last().unwrap().1;
    let space_required = space_to_delete - free_space;

    println!("space required = {} - {:?} = {}", space_to_delete, final_countdown.last().unwrap(), space_required);
    part_2_total = final_countdown
        .into_iter()
        .filter(move |(_, amount)| *amount >= space_required)
        .map(|(_, amount)| {
            return amount;
        })
        .min().unwrap();

    println!("part_1_total {part_1_total:?}");
    println!("part_2_total {part_2_total:?}");

    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    return None;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(48381165));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
