use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

const INPUT: &str = include_str!("day7.txt");

fn main() {
    let input = parse(INPUT);
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &FileSystem) -> u64 {
    input
        .dirs
        .iter()
        .map(|(path, _)| input.path_sizes.get(path).copied().unwrap_or(0))
        .filter(|&it| it <= 100_000)
        .sum()
}

fn part2(input: &FileSystem) -> u64 {
    const TOTAL_SPACE: u64 = 70_000_000;
    const MIN_SPACE: u64 = 30_000_000;

    let currently_used = input
        .path_sizes
        .get(&PathBuf::from("/"))
        .copied()
        .expect("expecting size of /");
    let unused = TOTAL_SPACE
        .checked_sub(currently_used)
        .expect("TOTAL_SPACE - currently_used overflow");
    let to_free = MIN_SPACE
        .checked_sub(unused)
        .expect("MIN_SPACE - unused overflow");

    input
        .dirs
        .iter()
        .filter_map(|(path, _)| {
            let size = input.path_sizes.get(path).copied().unwrap_or(0);
            let enough = size >= to_free;
            if enough {
                // println!("{path:?} has size {size}");
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("expecting a minimum size dir")
}

#[derive(Debug, PartialEq, Eq)]
struct FileSystem {
    // subdirs/files are not in a Vec, because each dir can only contain unique elements
    // that one was annoying to figure out...
    dirs: HashMap<PathBuf, HashSet<PathBuf>>,
    files: HashMap<PathBuf, u64>,
    path_sizes: HashMap<PathBuf, u64>,
}

fn parse(input: &str) -> FileSystem {
    let mut dirs = HashMap::new();
    let mut files = HashMap::new();
    let mut pwd = PathBuf::from("/"); // Start at root at all times.

    for execution in input.split("$") {
        let execution = execution.trim();
        if execution.is_empty() {
            continue;
        }
        // The first line will be a command, the rest (if any) are output.
        let command = execution.lines().next().expect("expected a command line");
        let output = execution.lines().skip(1);

        if command.starts_with("cd ") {
            // ignore output; it doesn't make sense to have any here

            let (_, path) = command.split_once(' ').expect("expected a path in cd");
            if path == "/" {
                pwd = PathBuf::from("/");
            } else if path == ".." {
                let _ = pwd.pop();
            } else {
                // A real path!
                let mut file_path = pwd.clone();
                file_path.push(path);
                dirs.entry(pwd.clone())
                    .or_insert_with(|| HashSet::with_capacity(1))
                    .insert(file_path);
                pwd.push(path); // don't add an entry in case nothing is there
            }
        } else if command == "ls" {
            for line in output {
                let (info, path) = line
                    .split_once(' ')
                    .expect("expected ls output line to conform to standard");
                let mut file_path = pwd.clone();
                file_path.push(path);
                dirs.entry(pwd.clone())
                    .or_insert_with(|| HashSet::with_capacity(1))
                    .insert(file_path.clone());

                if info == "dir" {
                    continue;
                }

                let size = info.parse().expect("expecting non-dir to be size of file");
                files.insert(file_path, size);
            }
        } else {
            panic!("unknown command: {command}");
        }
    }

    let mut fs = FileSystem {
        path_sizes: HashMap::with_capacity(dirs.len()),
        dirs,
        files,
    };

    fn calculate_dir_sizes(fs: &mut FileSystem, path: PathBuf) -> u64 {
        let mut size = 0;

        for entry in fs.dirs.get(&path).cloned().unwrap_or_default() {
            let current_file = path.join(entry);
            match fs.files.get(&current_file) {
                Some(&file_size) => size += file_size,
                // it's a directory!
                None => size += calculate_dir_sizes(fs, current_file),
            }
        }

        fs.path_sizes.insert(path, size);
        size
    }
    calculate_dir_sizes(&mut fs, PathBuf::from("/"));

    fs
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::{hashmap, hashset};
    use pretty_assertions::assert_eq;

    fn input() -> FileSystem {
        FileSystem {
            dirs: hashmap! {
                PathBuf::from("/") => hashset!{p("/a"), p("/b.txt"), p("/c.dat"), p("/d")},
                PathBuf::from("/a") => hashset!{p("/a/e"), p("/a/f"), p("/a/g"), p("/a/h.lst")},
                PathBuf::from("/a/e") => hashset!{p("/a/e/i")},
                PathBuf::from("/d") => hashset!{p("/d/j"), p("/d/d.log"), p("/d/d.ext"), p("/d/k")},
            },
            path_sizes: hashmap! {
                PathBuf::from("/") => 48381165,
                PathBuf::from("/a") => 94853,
                PathBuf::from("/a/e") => 584,
                PathBuf::from("/a/e/i") => 584,
                PathBuf::from("/a/f") => 29116,
                PathBuf::from("/a/g") => 2557,
                PathBuf::from("/a/h.lst") => 62596,
                PathBuf::from("/b.txt") => 14848514,
                PathBuf::from("/c.dat") => 8504156,
                PathBuf::from("/d") => 24933642,
                PathBuf::from("/d/j") => 4060174,
                PathBuf::from("/d/d.log") => 8033020,
                PathBuf::from("/d/d.ext") => 5626152,
                PathBuf::from("/d/k") => 7214296,
            },
            files: hashmap! {
                PathBuf::from("/a/e/i") => 584,
                PathBuf::from("/a/f") => 29116,
                PathBuf::from("/a/g") => 2557,
                PathBuf::from("/a/h.lst") => 62596,
                PathBuf::from("/b.txt") => 14848514,
                PathBuf::from("/c.dat") => 8504156,
                PathBuf::from("/d/j") => 4060174,
                PathBuf::from("/d/d.log") => 8033020,
                PathBuf::from("/d/d.ext") => 5626152,
                PathBuf::from("/d/k") => 7214296,
            },
        }
    }

    #[test]
    fn part1_correct() {
        assert_eq!(part1(&input()), 95437);
    }

    #[test]
    fn part2_correct() {
        assert_eq!(part2(&input()), 24933642);
    }

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }
}
