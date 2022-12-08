use itertools::Itertools as _;

const INPUT: &str = include_str!("day8.txt");
type TreeHeight = usize;

fn main() {
    let input = parse(INPUT);
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &[Vec<TreeHeight>]) -> usize {
    let row_len = input[0].len();
    let mut visible = row_len /* top */ + row_len /* bottom */ + input.len() - 2 /* left side */ + input.len() - 2 /* right side */;
    // There is nothing to read on the first line, because they won't be fully covered.
    for (idxr, row) in input.iter().enumerate().skip(1) {
        // Also nothing on the last line will be visible.
        if idxr == input.len() - 1 {
            continue;
        }

        // There is nothing to read on the first column, because they won't be fully covered.
        for (idxc, &col) in row.iter().enumerate().skip(1) {
            // Also nothing on the last column will be visible.
            if idxc == row.len() - 1 {
                continue;
            }

            let column = (0..input.len()).map(|row| input[row][idxc]).collect_vec();

            let left_edge = &row[..idxc];
            let right_edge = &row[idxc + 1..];
            let up_edge = &column[..idxr];
            let down_edge = &column[idxr + 1..];

            if [up_edge, down_edge, left_edge, right_edge]
                .into_iter()
                .any(|to_edge| to_edge.iter().all(|&h| col > h))
            {
                visible += 1;
            }
        }
    }
    visible
}

fn part2(input: &[Vec<TreeHeight>]) -> usize {
    let mut max_score = 0;

    for (idxr, row) in input.iter().enumerate() {
        for (idxc, &col) in row.iter().enumerate() {
            let column = (0..input.len()).map(|row| input[row][idxc]).collect_vec();

            let mut left_edge = row[..idxc].to_vec();
            left_edge.reverse();
            let right_edge = row[idxc + 1..].to_vec();
            let mut up_edge = column[..idxr].to_vec();
            up_edge.reverse();
            let down_edge = column[idxr + 1..].to_vec();

            if [&up_edge, &down_edge, &left_edge, &right_edge]
                .into_iter()
                .any(|to_edge| to_edge.iter().all(|&h| col > h))
            {
                // This is a valid tree. Let's find its scenic score.
                let score = [&left_edge, &right_edge, &up_edge, &down_edge]
                    .into_iter()
                    .map(|to_edge| {
                        let mut count = 0;
                        for &tree in to_edge {
                            count += 1;
                            if tree >= col {
                                // We can still see it, so we'll leave count alone.
                                break;
                            }
                        }
                        count
                    })
                    .product();
                max_score = std::cmp::max(max_score, score);
            }
        }
    }

    max_score
}

fn parse(input: &str) -> Vec<Vec<TreeHeight>> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(usize::try_from)
                        .map(Result::ok)
                        .flatten()
                        .ok_or_else(|| format!("got non-digit {c} for line: {line}"))
                        .expect("expected lines only containing digits")
                })
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn input() -> Vec<Vec<TreeHeight>> {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn parse_correct() {
        assert_eq!(
            parse(
                "30373
25512
65332
33549
35390"
            ),
            input()
        );
    }

    #[test]
    fn part1_correct() {
        assert_eq!(part1(&input()), 21);
    }

    #[test]
    fn part2_correct() {
        assert_eq!(part2(&input()), 8);
    }
}
