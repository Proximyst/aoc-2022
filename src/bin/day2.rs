const INPUT: &str = include_str!("day2.txt");
const ROCK: i64 = 0;
const PAPER: i64 = 1;
const SCISSOR: i64 = 2;
const SHOULD_LOSE: i64 = 0;
const SHOULD_TIE: i64 = 1;
const SHOULD_WIN: i64 = 2;
const TIE: i64 = 3;
const WIN: i64 = 6;

fn main() {
    let choices = parse(INPUT);
    println!("Part 1: {:?}", part1(&choices));
    println!("Part 2: {:?}", part2(&choices));
}

fn part1(choices: &[(i64, i64)]) -> i64 {
    choices
        .into_iter()
        .map(|&round| {
            let result = if round.0 == round.1 {
                TIE
            } else if round.0 == ROCK {
                (round.1 - 2) * WIN
            } else if round.0 == PAPER {
                round.1 * 3
            } else if round.0 == SCISSOR {
                (round.1 - 1) * 6
            } else {
                panic!("unknown round: {:?}", round)
            }
            .abs();
            round.1 + 1 + result
        })
        .sum()
}

fn part2(choices: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for &(elf, expected) in choices {
        let you = match (elf, expected) {
            (ROCK, SHOULD_LOSE) => SCISSOR,
            (ROCK, SHOULD_WIN) => PAPER,
            (PAPER, SHOULD_LOSE) => ROCK,
            (PAPER, SHOULD_WIN) => SCISSOR,
            (SCISSOR, SHOULD_LOSE) => PAPER,
            (SCISSOR, SHOULD_WIN) => ROCK,
            (_, SHOULD_TIE) => elf,
            _ => panic!(),
        };
        sum += you + 1 + (expected * 3);
    }
    sum
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut vec = Vec::new();
    for line in input.lines() {
        let &[a, _, b] = line.as_bytes() else { panic!("invalid line: {}", line) };
        vec.push((a as i64 - b'A' as i64, b as i64 - b'X' as i64));
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_correctly() {
        assert_eq!(parse("A X\nB Y\nC Z"), vec![(0, 0), (1, 1), (2, 2)]);
    }

    #[test]
    fn part1_correct() {
        assert_eq!(part1(&[(ROCK, PAPER), (PAPER, ROCK), (SCISSOR, SCISSOR)]), 15);
    }

    #[test]
    fn part2_correct() {
        assert_eq!(part2(&[(ROCK, PAPER), (PAPER, ROCK), (SCISSOR, SCISSOR)]), 12);
    }
}
