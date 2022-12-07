
pub fn part_one(input: &str) -> Option<u32> {
    let mut stacks = init_stack(input);

    // let mut stacks: Vec<Vec<String>> = vec![vec![]; 9];

    println!("{stacks:?}");
    let mut start = 0;
    for line in input.lines() {
        if line == "" {
            start = 1;
            continue;
        }
        if start == 0 {
            continue;
        }
        let vec = line.split(" ")
            .filter(|c| { c != &"move" && c != &"to" && c != &"from"})
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let amount: usize = vec[0].try_into().unwrap();
        let mut stack_from: usize = vec[1].try_into().unwrap(); //0 based index remeber :)
        let mut stack_to: usize = vec[2].try_into().unwrap(); //0 based index remeber :)
        stack_from = stack_from - 1;
        stack_to = stack_to - 1;
        for _ in 0..amount {
            let thing = &stacks[stack_from].pop().unwrap();
            stacks[stack_to].push(thing.to_string());
        }
    }
    let mut answer = String::from("");
    for stack in stacks {
        answer = answer + &stack[stack.len() - 1];
    }
    println!("Answer: {answer:?}");
    None
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stacks = init_stack(input);

    println!("{stacks:?}");
    let mut start = 0;
    for line in input.lines() {
        if line == "" {
            start = 1;
            continue;
        }
        if start == 0 {
            continue;
        }
        println!("{line:?}");
        let vec = line.split(" ")
            .filter(|c| { c != &"move" && c != &"to" && c != &"from"})
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let amount: usize = vec[0].try_into().unwrap();
        let mut stack_from: usize = vec[1].try_into().unwrap(); //0 based index remeber :)
        let mut stack_to: usize = vec[2].try_into().unwrap(); //0 based index remeber :)
        stack_from = stack_from - 1;
        stack_to = stack_to - 1;

        let len = stacks[stack_from].len() - amount;
        let n = stacks[stack_from].drain(len..).collect::<Vec<String>>();
        stacks[stack_to].extend(n.iter().cloned());
        println!("{n:?}");
    }
    let mut answer = String::from("");
    for stack in stacks {
        if stack.len() > 0  {
            answer = answer + &stack[stack.len() - 1];
        }
    }
    println!("Answer: {answer:?}");
    None
}

fn init_stack(input: &str) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = vec![vec![]; 9];
    for line in input.lines() {
        if line == "" {
            break;
        }
        let mut index = 0;
        for char in line.chars() {
            if char == '1' { break; }
            if char != '[' && char != ']' && char != ' ' {
                stacks[(index + 3) / 4 - 1].push(char.to_string());
            }
            index = index + 1;
        }
    }
    let mut rev_stacks: Vec<Vec<String>> = vec![];
    for stack in stacks {
        let mut st = stack.clone();
        st.reverse();
        rev_stacks.push(st);
    }
    return rev_stacks;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let input = advent_of_code::read_file("examples", 5);
    //     assert_eq!(part_one(&input), None);
    // }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
