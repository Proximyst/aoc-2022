use itertools::Itertools as _;

const INPUT: &str = include_str!("day6.txt");

fn main() {
    println!("Part 1: {:?}", part1(INPUT));
    println!("Part 2: {:?}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    for i in 0..input.len() - 4 {
        let slice = &input[i..i + 4];
        if slice.chars().unique().count() == 4 {
            return i + 4;
        }
    }
    panic!("no start of packet in: {input}")
}

fn part2(input: &str) -> usize {
    for i in 0..input.len() - 14 {
        let slice = &input[i..i + 14];
        if slice.chars().unique().count() == 14 {
            return i + 14;
        }
    }
    panic!("no start of packet in: {input}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_correct() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_correct() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
