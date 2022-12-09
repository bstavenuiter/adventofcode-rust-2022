#[macro_export]
macro_rules! tree_me {
    ( $x:ident, $y:ident, $dir_value:ident, $dir_max:ident, $seen:ident ) => {{
        if $dir_value > $dir_max {
            $seen[$y][$x] += 1;
            $dir_max = $dir_value;
        }
    }};
}

#[macro_export]
macro_rules! tree_2 {
    ( $x:ident, $y:ident, $height:ident, $trees:ident, $out:ident ) => {{
        if $trees[$y][$x] < $height {
            $out += 1;
        } else if $trees[$y][$x] == $height {
            $out += 1;
            break;
        } else {
            break;
        }
    }};
}

fn see_tree(trees: &Vec<Vec<isize>>, x: usize, y: usize) -> usize {
    let w = trees[0].len();
    let h = trees.len();

    let height = trees[y][x];
    let mut out = 1;

    let mut temp = 0;
    for x in (0..x).rev() {
        tree_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for x in x + 1..w {
        tree_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for y in (0..y).rev() {
        tree_2!(x, y, height, trees, temp);
    }
    out *= temp;

    let mut temp = 0;
    for y in y + 1..h {
        tree_2!(x, y, height, trees, temp);
    }
    out *= temp;

    return out;
}

pub fn part_one(input: &str) -> Option<usize> {
    // Example
    // 30373
    // 25512
    // 65332
    // 33549
    // 35390

    let trees = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            return line
                .chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as isize)
                .collect::<Vec<isize>>();
        })
        .collect::<Vec<Vec<isize>>>();

    let h = trees.len();
    let w = trees[0].len();
    let mut seen = vec![vec![0usize; w]; h];

    for y in 0..h {
        let mut e_h = -1;
        let mut w_h = -1;
        for x in 0..w {
            let w_idx = w - x - 1;
            let west = trees[y][w_idx];
            let east = trees[y][x];

            tree_me!(x, y, east, e_h, seen);
            tree_me!(w_idx, y, west, w_h, seen);
        }
    }

    for x in 0..w {
        let mut n_h = -1;
        let mut s_h = -1;
        for y in 0..h {
            let n_idx = h - y - 1;
            let south = trees[y][x];
            let north = trees[n_idx][x];

            tree_me!(x, y, south, s_h, seen);
            tree_me!(x, n_idx, north, n_h, seen);
        }
    }

    let amount_visible = seen
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x != 0)
        .count();

    return Some(amount_visible * 1);
}

pub fn part_two(input: &str) -> Option<isize> {
    let trees = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            return line
                .chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as isize)
                .collect::<Vec<isize>>();
        })
        .collect::<Vec<Vec<isize>>>();

    let h = trees.len();
    let w = trees[0].len();
    let mut seen = vec![vec![0isize; w]; h];

    for y in 0..h {
        for x in 0..w {
            seen[y][x] = see_tree(&trees, x, y) as isize;
        }
    }
    let score = seen.iter().flat_map(|x| x.iter()).max().unwrap();

    // println!("{:?}", seen);
    println!("{:?}", score);

    // too low 345600
    // 374400
    // too high 5762400

    return Some(*score);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
