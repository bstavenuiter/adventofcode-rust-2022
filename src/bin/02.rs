use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {

    let mut total_score = 0;

    for line in input.lines() {
        let vec = line.split(" ").collect::<Vec<&str>>();
        total_score = total_score + check(vec[0], vec[1]);
    }
    println!("{}", total_score);
    return Some(total_score);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score_part_2 = 0;
    for line in input.lines() {
        let vec = line.split(" ").collect::<Vec<&str>>();
        total_score_part_2 = total_score_part_2 + check_part2(vec[0], vec[1]);
    }
    println!("part2: {}", total_score_part_2);
    return Some(total_score_part_2);
}

fn check(opp: &str, me: &str) -> u32 {
    let choise_points = HashMap::from([
        ( "X", 1 ), //rock
        ( "Y", 2 ), //paper
        ( "Z", 3 ), //scissors
    ]);
    let outcome_points = HashMap::from([
        ( "X", 0 ), //loose
        ( "Y", 3 ), //draw
        ( "Z", 6 ), //win
    ]);

    match opp {
        "A" => { //rock
            match me {
                "X" => choise_points["X"] + outcome_points["Y"], //rock
                "Y" => choise_points["Y"] + outcome_points["Z"], //paper
                "Z" => choise_points["Z"] + outcome_points["X"], //scissors
                &_ => panic!("No correct result"),
            }
        }
        "B" => { //paper
            match me {
                "X" => choise_points["X"] + outcome_points["X"], //rock
                "Y" => choise_points["Y"] + outcome_points["Y"], //paper
                "Z" => choise_points["Z"] + outcome_points["Z"], //scissors
                &_ => panic!("No correct result"),
            }
        }
        "C" => { //scissors
            match me {
                "X" => choise_points["X"] + outcome_points["Z"], //rock
                "Y" => choise_points["Y"] + outcome_points["X"], //paper
                "Z" => choise_points["Z"] + outcome_points["Y"], //scissors
                &_ => panic!("No correct result"),
            }
        },
        &_ => panic!("No correct result"),
    }
}

fn check_part2(opp: &str, me: &str) -> u32 {
    match opp {
        "A" => { //rock
            match me {
                "X" => 3 + 0, //loose, choose scissors
                "Y" => 1 + 3, //draw, choose rock
                "Z" => 2 + 6, //win, choose paper
                &_ => panic!("No correct result"),
            }
        }
        "B" => { //paper
            match me {
                "X" => 1 + 0, //loose, rock
                "Y" => 2 + 3, //draw paper
                "Z" => 3 + 6, //win, scissors
                &_ => panic!("No correct result"),
            }
        }
        "C" => { //scissors
            match me {
                "X" => 2 + 0, //loose, paper
                "Y" => 3 + 3, //draw, scissors
                "Z" => 1 + 6, //win, rock
                &_ => panic!("No correct result"),
            }
        },
        &_ => panic!("No correct result"),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
