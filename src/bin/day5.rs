use itertools::Itertools as _;
use std::collections::VecDeque;

const INPUT: &str = include_str!("day5.txt");

fn main() {
    let input = parse(INPUT);
    println!("Part 1: {:?}", part1(input.clone()));
    println!("Part 2: {:?}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    stacks: VecDeque<VecDeque<char>>,
    instructions: VecDeque<Move>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn part1(mut input: Input) -> String {
    for mv in input.instructions {
        for i in 0..mv.count {
            let moving = input.stacks[mv.from - 1]
                .pop_back()
                .ok_or_else(|| format!("for {i} of {mv:?}, could not perform move"))
                .unwrap();

            input.stacks[mv.to - 1].push_back(moving);
        }
    }

    input
        .stacks
        .into_iter()
        .map(|mut s| s.pop_back())
        .flatten()
        .join("")
}

fn part2(mut input: Input) -> String {
    for mv in input.instructions {
        if mv.count == 1 {
            let moving = input.stacks[mv.from - 1]
                .pop_back()
                .ok_or_else(|| format!("for {mv:?}, could not perform move"))
                .unwrap();

            input.stacks[mv.to - 1].push_back(moving);
            continue;
        }

        let from = &mut input.stacks[mv.from - 1];
        let mut moving = from.split_off(from.len() - mv.count);
        input.stacks[mv.to - 1].append(&mut moving);
    }

    input
        .stacks
        .into_iter()
        .map(|mut s| s.pop_back())
        .flatten()
        .join("")
}

fn parse(input: &str) -> Input {
    let (in_crates, in_instructions) = input
        .split_once("\n\n")
        .expect("there must be a \\n\\n in the input");

    // in_crates looks like:
    //
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    // This finds how many columns are in the last line, which enumerates the graph.
    let columns = in_crates
        .lines()
        .last()
        .expect("expecting a number line as last of crates list")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10))
        .flatten()
        .max()
        .expect("expecting a max number in number line") as usize;
    let mut stacks = VecDeque::with_capacity(columns);
    for _ in 0..columns {
        stacks.push_back(VecDeque::new());
    }
    // Going bottom up, the first line is just integers we don't care about.
    for line in in_crates.lines().rev().skip(1) {
        for i in 0..columns {
            let idx = 1 + 4 * i;
            let ch = line.chars().nth(idx).expect("expecting a char in line");
            if ch.is_alphabetic() {
                stacks[i].push_back(ch);
            }
        }
    }

    // in_instructions look like:
    //
    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2
    let mut instructions = VecDeque::new();
    for line in in_instructions.lines() {
        let (_, cnt, _, from, _, to) = line
            .split(' ')
            .collect_tuple()
            .expect("expecting instruction line to conform to standard");
        let cnt = cnt.parse().expect("expecting cnt to be integer");
        let from = from.parse().expect("expecting from to be integer");
        let to = to.parse().expect("expecting to to be integer");

        instructions.push_back(Move {
            count: cnt,
            from,
            to,
        });
    }

    Input {
        stacks,
        instructions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_correctly() {
        assert_eq!(
            parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            ),
            Input {
                stacks: deq([deq(['Z', 'N']), deq(['M', 'C', 'D']), deq(['P'])]),
                instructions: deq([mv(1, 2, 1), mv(3, 1, 3), mv(2, 2, 1), mv(1, 1, 2)]),
            }
        );
    }

    #[test]
    fn part1_correct() {
        assert_eq!(
            part1(Input {
                stacks: deq([deq(['Z', 'N']), deq(['M', 'C', 'D']), deq(['P'])]),
                instructions: deq([mv(1, 2, 1), mv(3, 1, 3), mv(2, 2, 1), mv(1, 1, 2)]),
            }),
            "CMZ"
        );
    }

    #[test]
    fn part2_correct() {
        assert_eq!(
            part2(Input {
                stacks: deq([deq(['Z', 'N']), deq(['M', 'C', 'D']), deq(['P'])]),
                instructions: deq([mv(1, 2, 1), mv(3, 1, 3), mv(2, 2, 1), mv(1, 1, 2)]),
            }),
            "MCD"
        );
    }

    fn mv(count: usize, from: usize, to: usize) -> Move {
        Move { count, from, to }
    }

    fn deq<const N: usize, T>(elem: [T; N]) -> VecDeque<T> {
        let mut deque = VecDeque::with_capacity(N);
        for e in elem {
            deque.push_back(e);
        }
        deque
    }
}
