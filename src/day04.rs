use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for (r, row) in inputs.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col == &b'X' {
                // right
                if row.get(c + 1) == Some(&b'M')
                    && row.get(c + 2) == Some(&b'A')
                    && row.get(c + 3) == Some(&b'S')
                {
                    count += 1;
                }

                // left
                if row.get(c.wrapping_sub(1)) == Some(&b'M')
                    && row.get(c.wrapping_sub(2)) == Some(&b'A')
                    && row.get(c.wrapping_sub(3)) == Some(&b'S')
                {
                    count += 1;
                }

                // down
                if inputs.get(r + 1).and_then(|r2| r2.get(c)) == Some(&b'M')
                    && inputs.get(r + 2).and_then(|r2| r2.get(c)) == Some(&b'A')
                    && inputs.get(r + 3).and_then(|r2| r2.get(c)) == Some(&b'S')
                {
                    count += 1;
                }

                // up
                if inputs.get(r.wrapping_sub(1)).and_then(|r2| r2.get(c)) == Some(&b'M')
                    && inputs.get(r.wrapping_sub(2)).and_then(|r2| r2.get(c)) == Some(&b'A')
                    && inputs.get(r.wrapping_sub(3)).and_then(|r2| r2.get(c)) == Some(&b'S')
                {
                    count += 1;
                }

                // right+down
                if inputs.get(r + 1).and_then(|r2| r2.get(c + 1)) == Some(&b'M')
                    && inputs.get(r + 2).and_then(|r2| r2.get(c + 2)) == Some(&b'A')
                    && inputs.get(r + 3).and_then(|r2| r2.get(c + 3)) == Some(&b'S')
                {
                    count += 1;
                }

                // right+up
                if inputs.get(r + 1).and_then(|r2| r2.get(c.wrapping_sub(1))) == Some(&b'M')
                    && inputs.get(r + 2).and_then(|r2| r2.get(c.wrapping_sub(2))) == Some(&b'A')
                    && inputs.get(r + 3).and_then(|r2| r2.get(c.wrapping_sub(3))) == Some(&b'S')
                {
                    count += 1;
                }

                // left+down
                if inputs.get(r.wrapping_sub(1)).and_then(|r2| r2.get(c + 1)) == Some(&b'M')
                    && inputs.get(r.wrapping_sub(2)).and_then(|r2| r2.get(c + 2)) == Some(&b'A')
                    && inputs.get(r.wrapping_sub(3)).and_then(|r2| r2.get(c + 3)) == Some(&b'S')
                {
                    count += 1;
                }

                // left+up
                if inputs
                    .get(r.wrapping_sub(1))
                    .and_then(|r2| r2.get(c.wrapping_sub(1)))
                    == Some(&b'M')
                    && inputs
                        .get(r.wrapping_sub(2))
                        .and_then(|r2| r2.get(c.wrapping_sub(2)))
                        == Some(&b'A')
                    && inputs
                        .get(r.wrapping_sub(3))
                        .and_then(|r2| r2.get(c.wrapping_sub(3)))
                        == Some(&b'S')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn get(inputs: &[Vec<u8>], r: usize, c: usize) -> [Option<&u8>; 4] {
    [
        inputs
            .get(r.wrapping_sub(1))
            .and_then(|r2| r2.get(c.wrapping_sub(1))),
        inputs
            .get(r.wrapping_sub(1))
            .and_then(|r2| r2.get(c.wrapping_add(1))),
        inputs
            .get(r.wrapping_add(1))
            .and_then(|r2| r2.get(c.wrapping_add(1))),
        inputs
            .get(r.wrapping_add(1))
            .and_then(|r2| r2.get(c.wrapping_sub(1))),
    ]
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let mut count = 0;

    for (r, row) in inputs.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col == &b'A' {
                let v = get(inputs, r, c);
                if v == [Some(&b'M'), Some(&b'S'), Some(&b'S'), Some(&b'M')]
                    || v == [Some(&b'S'), Some(&b'S'), Some(&b'M'), Some(&b'M')]
                    || v == [Some(&b'S'), Some(&b'M'), Some(&b'M'), Some(&b'S')]
                    || v == [Some(&b'M'), Some(&b'M'), Some(&b'S'), Some(&b'S')]
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
