use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::matrix::directions;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn get(inputs: &[Vec<u8>], r: usize, c: usize, (r_step, c_step): (isize, isize)) -> Option<u8> {
    let r = r.checked_add_signed(r_step)?;
    let c = c.checked_add_signed(c_step)?;

    inputs.get(r).and_then(|row| row.get(c)).copied()
}

fn check_mas(inputs: &[Vec<u8>], r: usize, c: usize, (r_step, c_step): (isize, isize)) -> bool {
    (1..4)
        .map(|step| get(inputs, r, c, (r_step * step, c_step * step)))
        .eq("MAS".bytes().map(Some))
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for (r, row) in inputs.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col == &b'X' {
                count += usize::from(check_mas(inputs, r, c, directions::N));
                count += usize::from(check_mas(inputs, r, c, directions::E));
                count += usize::from(check_mas(inputs, r, c, directions::S));
                count += usize::from(check_mas(inputs, r, c, directions::W));

                count += usize::from(check_mas(inputs, r, c, directions::NE));
                count += usize::from(check_mas(inputs, r, c, directions::SE));
                count += usize::from(check_mas(inputs, r, c, directions::SW));
                count += usize::from(check_mas(inputs, r, c, directions::NW));
            }
        }
    }

    count
}

fn get_corners(inputs: &[Vec<u8>], r: usize, c: usize) -> [Option<u8>; 4] {
    [
        get(inputs, r, c, directions::NE),
        get(inputs, r, c, directions::SE),
        get(inputs, r, c, directions::SW),
        get(inputs, r, c, directions::NW),
    ]
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let mut count = 0;

    for (r, row) in inputs.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col == &b'A' {
                let v = get_corners(inputs, r, c);
                if v == [Some(b'M'), Some(b'S'), Some(b'S'), Some(b'M')]
                    || v == [Some(b'S'), Some(b'S'), Some(b'M'), Some(b'M')]
                    || v == [Some(b'S'), Some(b'M'), Some(b'M'), Some(b'S')]
                    || v == [Some(b'M'), Some(b'M'), Some(b'S'), Some(b'S')]
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 18);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 9);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day4.txt");
        const ANSWERS: (usize, usize) = (2493, 1890);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
