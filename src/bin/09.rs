use std::{vec, collections::HashMap, arch::aarch64::vbic_s8, cmp::Ordering};

pub fn part_one(input: &str) -> Option<u32> {

    let mut visited = HashMap::new();
                  //x, y
    let mut head = (0, 0);
    let mut tail = (0, 0);

    visited.insert(format!("{}-{}", 0, 0), 1);

    for line in input.lines() {
        let (dir, mut amount) = line.split_once(" ")
            .map(|(x,y)| {
                (x, y.parse::<i32>().unwrap())
            }).unwrap();

        while amount > 0 {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => panic!("dir unknown")
            }
            tail = move_tail(tail, &head);
            visited.insert(format!("{}-{}", tail.0, tail.1), 1);
            amount -= 1;
        };
    }

    println!("{}", visited.len());
    Some(visited.len() as u32)
}

pub fn move_tail(mut tail: (i32, i32), head: &(i32, i32)) -> (i32, i32) {
    //only move when a distance is greater than 2
    let map = HashMap::from([
        (Ordering::Less, -1),
        (Ordering::Equal, 0),
        (Ordering::Greater, 1),
    ]);

    if (head.0 - tail.0).abs() > 1 
        || (head.1 - tail.1).abs() > 1 {

        tail.1 += map[&head.1.cmp(&tail.1)];
        tail.0 += map[&head.0.cmp(&tail.0)];
    }

    return tail;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited = HashMap::new();
                  //x, y
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];

    visited.insert(format!("{}-{}", 0, 0), 1);
    for line in input.lines() {
        let (dir, mut amount) = line.split_once(" ")
            .map(|(x,y)| {
                (x, y.parse::<i32>().unwrap())
            }).unwrap();

        while amount > 0 {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => panic!("dir unknown")
            }
            for x in 0..tails.len() {
                let mut my_head = &head;
                if x > 0 {
                    my_head = &tails[x-1];
                }
                tails[x] = move_tail(tails[x], &my_head);
            }
            visited.insert(format!("{}-{}", tails[8].0, tails[8].1), 1);
            amount -= 1;
        };
    }

    println!("{}", visited.len());

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_larger() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&input), Some(36));
    }

    #[test]
    fn test_tail_move() {
        //R
        assert_eq!(move_tail((0,0), &(2,0)), (1, 0));

        //L
        assert_eq!(move_tail((0,0), &(-2,0)), (-1, 0));

        //U
        assert_eq!(move_tail((0,0), &(0,2)), (0, 1));

        //D
        assert_eq!(move_tail((0,0), &(0,-2)), (0, -1));

        //one step after moving up, tail should move diagonally
        assert_eq!(move_tail((3,0), &(4,-2)), (4, -1));

        //diag
        assert_eq!(move_tail((3,0), &(4,-1)), (3, 0));
    }
}
