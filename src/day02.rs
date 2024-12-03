use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[Vec<i32>]) -> usize {
    inputs.iter().filter(|line| is_good(line)).count()
}

fn is_good(v: &[i32]) -> bool {
    v.windows(2).all(|x| {
        let d1 = x[0] - x[1];
        (1..4).contains(&d1)
    }) || v.windows(2).all(|x| {
        let d1 = x[1] - x[0];
        (1..4).contains(&d1)
    })
}

#[aoc(day2, part2, init)]
pub fn part2_init(inputs: &[Vec<i32>]) -> usize {
    inputs
        .iter()
        .filter(|line| {
            let part1 = is_good(line);

            if !part1 {
                for i in 0..line.len() {
                    let mut temp = line.to_vec();
                    temp.remove(i);
                    if is_good(&temp) {
                        return true;
                    }
                }
            }
            part1
        })
        .count()
}

fn is_good2<'a>(v: impl Iterator<Item = &'a i32> + Clone) -> bool {
    v.clone().tuple_windows().all(|t: (&i32, &i32)| {
        let d1 = t.0 - t.1;
        (1..4).contains(&d1)
    }) || v.tuple_windows().all(|t: (&i32, &i32)| {
        let d1 = t.1 - t.0;
        (1..4).contains(&d1)
    })
}

#[aoc(day2, part2, iter)]
pub fn part2_iter(inputs: &[Vec<i32>]) -> usize {
    inputs
        .iter()
        .filter(|line| {
            is_good2(line.iter())
                || (0..line.len())
                    .any(|i| is_good2(line.iter().take(i).chain(line.iter().skip(i + 1))))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2_init(&generator(SAMPLE)), 4);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2024/day2.txt");
        const ANSWERS: (usize, usize) = (670, 700);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2_init(&output), ANSWERS.1);
        }
    }
}
