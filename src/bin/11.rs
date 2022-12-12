pub struct Monkey<'a> {
    items: Vec<usize>,
    op : Vec<&'a str>,
    test: usize,
    action: (i32, i32),
    items_inspected: u32,
}

impl Monkey<'_> {

    fn new() -> Monkey<'static> {
        Monkey { 
            items: vec![],
            op: vec![],
            test: 0,
            action: (0i32,0i32),
            items_inspected: 0,
        }
    }

    fn add_worry(monkey: &mut Monkey, super_mod: usize) {
        for (_,s) in monkey.items.iter_mut().enumerate() {
            monkey.items_inspected += 1;
            match monkey.op[1] {
                "+" => {
                    *s += monkey.op[2].parse::<usize>().unwrap();
                },
                "-" => {
                    *s -= monkey.op[2].parse::<usize>().unwrap();
                },
                "*" => {
                    if monkey.op[2] == "old" {
                        *s *= *s;
                    } else {
                        *s *= monkey.op[2].parse::<usize>().unwrap();
                    }
                },
                _ => panic!("unknown operation")
            }
            if super_mod > 0 {
                *s = *s % super_mod;
            } else {
                *s = *s / 3;
            }
        }
    }

    fn throw_to(i: usize, monkeys: &mut Vec<Monkey>) {
        let test = monkeys[i].test;
        while let Some(item) = monkeys[i].items.pop() {
            if item % test == 0 {
                let index = monkeys[i].action.0 as usize;
                monkeys[index].items.push(item);
            } else {
                let index = monkeys[i].action.1 as usize;
                monkeys[index].items.push(item);
            }
        }
    }

}

pub fn parse_monkeys (input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut line_nr = 0;
    let mut monkey_nr = 0;

    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey::new());
            line_nr = 0;
        }

        match line_nr {
            1 => { //starting items
                monkeys[monkey_nr].items = line[18..]
                    .split(" ")
                    .map(|x| x.replace(",", "").parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            },
            2 => {
                monkeys[monkey_nr].op = line[19..].split(" ").collect::<Vec<&str>>();
            },
            3 => {
                monkeys[monkey_nr].test = line.split(" ").last().unwrap().parse::<usize>().unwrap();
            },
            4 => {
                monkeys[monkey_nr].action.0 = line.split(" ")
                        .last()
                        .unwrap().parse::<i32>()
                        .unwrap();
            },
            5 => {
                monkeys[monkey_nr].action.1 =                     line.split(" ").
                        last()
                        .unwrap().parse::<i32>()
                        .unwrap();
                monkey_nr += 1;
            },
            _ => {}
        }
        line_nr += 1;
    }
    return monkeys;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = parse_monkeys(input);
    let amount_rounds = 20;
    for _ in 0..amount_rounds {
        for i in 0..monkeys.len (){
            Monkey::add_worry(&mut monkeys[i], 0);
            Monkey::throw_to(i, &mut monkeys);
        }
    }

    let mut items_counted = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u32>>();
    items_counted.sort();
    let monkey_business = items_counted.pop().unwrap() * items_counted.pop().unwrap();

    println!("{}", monkey_business);
    return Some(monkey_business);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse_monkeys(input);
    let amount_rounds = 10000;
    let super_mod = monkeys.iter().fold(1, |acc, m| acc * m.test);
    for _ in 0..amount_rounds {
        for i in 0..monkeys.len (){
            Monkey::add_worry(&mut monkeys[i], super_mod as usize);
            Monkey::throw_to(i, &mut monkeys);
        }
    }

    let mut items_counted = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u32>>();
    items_counted.sort();
    let monkey_business = u64::from(items_counted.pop().unwrap()) * u64::from(items_counted.pop().unwrap());

    println!("{}", monkey_business);
    return Some(monkey_business);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
