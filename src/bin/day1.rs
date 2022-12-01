use eyre::{eyre, Result, WrapErr as _};

const INPUT: &str = include_str!("day1.txt");

fn main() -> Result<()> {
    color_eyre::install()?;

    let elves = parse(INPUT)?;
    println!("Part 1: {:?}", part1(&elves)?);
    println!("Part 2: {:?}", part2(&elves)?);

    Ok(())
}

fn part1(elves: &[Elf]) -> Result<u64> {
    elves
        .into_iter()
        .max_by(|x, y| x.total.cmp(&y.total))
        .map(|elf| elf.total)
        .ok_or(eyre!("no max elf found?"))
}

fn part2(elves: &[Elf]) -> Result<u64> {
    let mut copy = elves.to_vec();
    copy.sort_by(|x, y| x.total.cmp(&y.total));
    copy.pop()
        .ok_or(eyre!("no last elf"))?
        .total
        .checked_add(copy.pop().ok_or(eyre!("no second to last elf"))?.total)
        .ok_or(eyre!("could not add elf totals"))?
        .checked_add(copy.pop().ok_or(eyre!("no third to last elf"))?.total)
        .ok_or(eyre!("could not add elf totals"))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Elf {
    total: u64,
    items: Vec<u64>,
}

fn parse(input: &str) -> Result<Vec<Elf>> {
    let mut vec = Vec::new();
    for group in input.split("\n\n") {
        let mut elf = Elf::default();
        for line in group.lines() {
            let calories: u64 = line
                .trim()
                .parse()
                .wrap_err_with(|| format!("invalid calories in line: {}", line))?;
            elf.items.push(calories);
            elf.total = elf.total.checked_add(calories).ok_or_else(|| {
                eyre!(
                    "could not add to total (total={}, calories={})",
                    elf.total,
                    calories
                )
            })?;
        }
        if !elf.items.is_empty() {
            vec.push(elf);
        }
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_correctly() {
        assert_eq!(
            parse("1\n1\n3\n\n3\n\n4").unwrap(),
            vec![elf([1, 1, 3]), elf([3]), elf([4])],
        );
    }

    #[test]
    fn part1_correct() {
        assert_eq!(
            part1(&[
                elf([1000, 2000, 3000]),
                elf([4000]),
                elf([5000, 6000]),
                elf([7000, 8000, 9000]),
                elf([10000]),
            ])
            .unwrap(),
            24000,
        );
    }

    #[test]
    fn part2_correct() {
        assert_eq!(
            part2(&[
                elf([1000, 2000, 3000]),
                elf([4000]),
                elf([5000, 6000]),
                elf([7000, 8000, 9000]),
                elf([10000]),
            ])
            .unwrap(),
            45000,
        );
    }

    fn elf<const N: usize>(items: [u64; N]) -> Elf {
        let items = items.to_vec();
        Elf {
            total: items.iter().sum(),
            items,
        }
    }
}
