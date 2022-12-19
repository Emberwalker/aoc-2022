use crate::prelude::*;

const BASE_BYTE_LOWER: u32 = 'a' as u32 - 1;
const BASE_BYTE_UPPER: u32 = 'A' as u32 - 1;

#[derive(Debug)]
#[derive(clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
struct Backpack<'a> {
    a: &'a str,
    b: &'a str,
}

pub fn run(args: Args) -> RootErr {
    debug!("Args: {:?}", args);
    let line_src = files::read_lines(args.file)?;
    let mut score: u32 = 0;
    let mut badge_score: u32 = 0;
    let mut backpacks_grouped: Vec<String> = Vec::new();
    for line in line_src {
        let ln = line?;
        let bpk = split_backpack(&ln);
        let dupes = get_common_items(&bpk);
        let dupe_score: u32 = dupes.iter().map(map_char_to_priority).sum();
        debug!("{:?} -> {:?} -> {:?}", bpk, dupes, dupe_score);
        score += dupe_score;

        backpacks_grouped.push(ln);
        if backpacks_grouped.len() == 3 {
            let ab: String = get_dupe_chars(&backpacks_grouped[0], &backpacks_grouped[1]).into_iter().collect();
            let bc: String = get_dupe_chars(&backpacks_grouped[1], &backpacks_grouped[2]).into_iter().collect();
            let all = get_dupe_chars(&ab, &bc);
            let group_badge_score: u32 = all.iter().map(map_char_to_priority).sum();
            debug!("Group badge: {:?} -> {}", all, group_badge_score);
            badge_score += group_badge_score;
            backpacks_grouped.drain(..);
        }
    }

    info!("Total priority of duplicated items: {}", score);
    info!("Total priority of badge items: {}", badge_score);

    Ok(())
}

fn split_backpack<'a>(line: &'a str) -> Backpack<'a> {
    let half = line.len() / 2;
    Backpack { a: &line[0..half], b: &line[half..] }
}

fn get_common_items(bpk: &Backpack) -> Vec<char> {
    get_dupe_chars(bpk.a, bpk.b)
}

fn map_char_to_priority(ch: &char) -> u32 {
    if ch.is_lowercase() {
        (*ch as u32) - BASE_BYTE_LOWER
    } else {
        (*ch as u32) - BASE_BYTE_UPPER + 26
    }
}

fn get_dupe_chars(a: &str, b: &str) -> Vec<char> {
    let mut a_chars: Vec<char> = a.chars().collect();
    a_chars.sort();
    a_chars.dedup();

    let b_chars: Vec<char> = b.chars().collect();
    let mut out: Vec<char> = Vec::new();

    for char in a_chars {
        if b_chars.contains(&char) {
            out.push(char)
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_backpack() {
        assert_eq!(Backpack { a: "ab", b: "cd" }, split_backpack("abcd"))
    }

    #[test]
    fn test_dupes_a() {
        assert_eq!(vec!['p'], get_common_items(&Backpack { a: "vJrwpWtwJgWr", b: "hcsFMMfFFhFp" }))
    }

    #[test]
    fn test_dupes_b() {
        assert_eq!(vec!['L'], get_common_items(&Backpack { a: "jqHRNqRjqzjGDLGL", b: "rsFMfFZSrLrFZsSL" }))
    }

    #[test]
    fn test_dupes_c() {
        assert_eq!(vec!['P'], get_common_items(&Backpack { a: "PmmdzqPrV", b: "vPwwTWBwg" }))
    }
}
