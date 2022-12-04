use std::ops::RangeInclusive;

const INPUT: &str = include_str!("day4.txt");

fn main() {
    let input = parse(INPUT);
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(ranges: &[(RangeInclusive<u64>, RangeInclusive<u64>)]) -> usize {
    ranges
        .into_iter()
        .filter(|(first, second)| {
            (first.start() >= second.start() && first.end() <= second.end())
                || (second.start() >= first.start() && second.end() <= first.end())
        })
        .count()
}

fn part2(ranges: &[(RangeInclusive<u64>, RangeInclusive<u64>)]) -> usize {
    ranges
        .into_iter()
        .filter(|(first, second)| {
            (first.start() >= second.start() && first.start() <= second.end())
                || (first.end() <= second.end() && first.end() >= second.start())
                || (second.start() >= first.start() && second.start() <= first.end())
                || (second.end() <= first.end() && second.end() >= first.start())
        })
        .count()
}

fn parse(input: &str) -> Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    input
        .lines()
        .map(|s| s.split_once(',').expect("invalid line (no comma)"))
        .map(|(first, second)| {
            fn parse_range(r: &str) -> RangeInclusive<u64> {
                let (from, to) = r.split_once('-').expect("no dash in range");
                let from: u64 = from.parse().expect("invalid from");
                let to: u64 = to.parse().expect("invalid to");
                assert!(from <= to, "from should be <= to");
                from..=to
            }

            (parse_range(first), parse_range(second))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_correctly() {
        assert_eq!(
            parse(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            ),
            vec![
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ],
        );
    }

    #[test]
    fn part1_correct() {
        assert_eq!(
            part1(&[
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]),
            2,
        );
    }

    #[test]
    fn part2_correct() {
        assert_eq!(
            part2(&[
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]),
            4,
        );
    }
}
