use itertools::Itertools as _;

const INPUT: &str = include_str!("day3.txt");

fn main() {
    let input = parse(INPUT);
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(rucksack: &[&str]) -> i64 {
    rucksack
        .into_iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| {
            a.chars()
                .filter(|&c| b.contains(c))
                .unique()
                .collect_tuple::<(char,)>()
                .ok_or_else(|| format!("could not find shared item in compartments: ({a}, {b})"))
                .unwrap()
                .0
        })
        .map(prio)
        .sum()
}

fn part2(rucksack: &[&str]) -> i64 {
    rucksack
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .copied()
                .collect_tuple::<(&str, &str, &str)>()
                .expect("could not find 3 items in chunk")
        })
        .map(|(a, b, c)| {
            a.chars()
                .filter(|&ch| b.contains(ch))
                .filter(|&ch| c.contains(ch))
                .unique()
                .collect_tuple::<(char,)>()
                .ok_or_else(|| format!("could not find shared item in rucksacks: ({a}, {b}, {c})"))
                .unwrap()
                .0
        })
        .map(prio)
        .sum()
}

fn prio(c: char) -> i64 {
    if c.is_lowercase() {
        c as i64 - b'a' as i64 + 1
    } else {
        c as i64 - b'A' as i64 + 27
    }
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|&line| {
            assert_eq!(
                line.len() % 2,
                0,
                "line is invalid line length (not even): {line}"
            );
            true
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_correctly() {
        assert_eq!(
            parse(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
        );
    }

    #[test]
    fn part1_correct() {
        assert_eq!(
            part1(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]),
            157,
        );
    }

    #[test]
    fn part2_correct() {
        assert_eq!(
            part2(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]),
            70,
        );
    }
}
