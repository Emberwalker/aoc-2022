use crate::prelude::*;

use std::io::Error;

#[derive(Debug)]
#[derive(clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

pub fn day1(args: Args) -> RootErr {
    debug!("Args: {:?}", args);
    let line_src = files::read_lines(args.file)?;
    let mut elves = sum_elves(line_src)?;
    // Sorting makes finding both puzzles easier
    elves.sort_unstable();
    debug!("Elves: {:?}", elves);

    info!("The elf with the most calories has {} calories", elves[elves.len() - 1]);
    if elves.len() >= 3 {
        info!("The top three elves have {} calories in total", elves.drain(elves.len() - 3..elves.len()).sum::<u32>());
    } else {
        warn!("There aren't enough elves to calculate the top three sum!")
    }

    Ok(())
}

fn sum_elves(line_src: impl Iterator<Item = Result<String, Error>>) -> ErrWrapper<Vec<u32>> {
    let mut out: Vec<u32> = Vec::new();
    let mut accum: Vec<u32> = Vec::new();
    for line_raw in line_src {
        let line = line_raw?;
        if line.len() == 0 {
            if !accum.is_empty() {
                out.push(accum.drain(..).sum());
            }
        } else {
            accum.push(line.parse()?)
        }
    }

    if !accum.is_empty() {
        out.push(accum.drain(..).sum())
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_elves_single() {
        let actual = sum_elves(read_lines_result_vec!["1", "2", "3"]).unwrap();
        assert_eq!(actual, vec![6]);
    }

    #[test]
    fn sum_elves_multi() {
        let actual = sum_elves(read_lines_result_vec!["1", "2", "", "5", "1", ""]).unwrap();
        assert_eq!(actual, vec![3, 6]);
    }
}
