use crate::prelude::*;

#[derive(Debug, clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        let other_range = other.start..=other.end;
        (self.start..=self.end).any(|r| other_range.contains(&r))
    }
}

impl TryFrom<&str> for Range {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once('-') {
            Some((start, end)) => Ok(Range {
                start: start.parse()?,
                end: end.parse()?,
            }),
            None => bail!("Unable to parse {} as a range", value),
        }
    }
}

pub fn run(args: Args) -> RootErr {
    debug!("Args: {:?}", args);
    let line_src = files::read_lines(args.file)?;

    let mut ranges: Vec<(Range, Range)> = Vec::new();
    for raw_line in line_src {
        let line = raw_line?;
        if !line.is_empty() {
            let (start, end) = match line.split_once(',') {
                Some((start, end)) => (start, end),
                None => bail!("Invalid line: {}", line),
            };
            ranges.push((start.try_into()?, end.try_into()?))
        }
    }

    let contains = ranges.iter().filter(|(start, end)| start.contains(end) || end.contains(start)).count();
    info!("{} pairs of shifts fully overlap", contains);

    let overlaps = ranges.iter().filter(|(start, end)| start.overlaps(end) || end.overlaps(start)).count();
    info!("{} pairs of shifts partially overlap", overlaps);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_contains_totally() {
        assert_eq!(
            true,
            Range { start: 0, end: 3 }.contains(&Range { start: 1, end: 2 })
        );
    }

    #[test]
    fn range_contains_equal() {
        assert_eq!(
            true,
            Range { start: 0, end: 1 }.contains(&Range { start: 0, end: 1 })
        );
    }

    #[test]
    fn range_contains_overlap() {
        assert_eq!(
            false,
            Range { start: 0, end: 1 }.contains(&Range { start: 1, end: 2 })
        );
    }

    #[test]
    fn range_parse() {
        assert_eq!(
            Range { start: 0, end: 1 },
            Range::try_from("0-1").unwrap()
        );
    }
}
