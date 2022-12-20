use once_cell::sync::Lazy;
use regex::Regex;

use crate::prelude::*;

#[derive(Debug, clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

type Crate = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransferOrder {
    amount: usize,
    source: usize,
    destination: usize,
}

static RE_TRANSFER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());

pub fn run_a(args: Args) -> RootErr {
    run(args, execute_mover_9000)
}

pub fn run_b(args: Args) -> RootErr {
    run(args, execute_mover_9001)
}

fn run(args: Args, crane_fn: impl Fn(&Vec<TransferOrder>, &mut Vec<Vec<Crate>>) -> RootErr) -> RootErr {
    debug!("Args: {:?}", args);

    // Parse stacks
    let stack_raw_lines: Vec<String> = files::read_lines(args.file.clone())?
        .map(Result::unwrap)
        .take_while(|s| !s.is_empty())
        .collect();
    let stack_section_line_count = stack_raw_lines.len(); // Used to skip the stacks header later
    let mut stacks = parse_crates(stack_raw_lines);

    // Parse movement orders
    let mut movements: Vec<TransferOrder> = Vec::new();
    for line_raw in files::read_lines(args.file)?.skip(stack_section_line_count) {
        let line = line_raw?;
        if line.is_empty() {
            continue;
        }
        let order = parse_transfer(&line)?;
        movements.push(order);
    }

    // Run move orders
    crane_fn(&movements, &mut stacks)?;

    print_stacks(&stacks);

    Ok(())
}

fn execute_mover_9000(movements: &Vec<TransferOrder>, stacks: &mut Vec<Vec<Crate>>) -> RootErr {
    for movement in movements {
        let src = movement.source - 1;
        let dst = movement.destination - 1;
        for _ in 0..movement.amount {
            let cr: Crate = stacks[src].pop().ok_or(simple_error!("Ordered to pick from empty stack!"))?;
            debug!("Moving crate {} from {} to {}.", cr, movement.source, movement.destination);
            stacks[dst].push(cr);
        }
    }
    Ok(())
}

fn execute_mover_9001(movements: &Vec<TransferOrder>, stacks: &mut Vec<Vec<Crate>>) -> RootErr {
    for movement in movements {
        let src = movement.source - 1;
        let dst = movement.destination - 1;
        let pop_index = stacks[src].len() - movement.amount;
        let mut split = stacks[src].split_off(pop_index);
        debug!("Moving crates {:?} from {} to {}.", split, movement.source, movement.destination);
        stacks[dst].append(&mut split)
    }
    Ok(())
}

fn parse_crates(mut stack_raw_lines: Vec<String>) -> Vec<Vec<Crate>> {
    let stacks_count = stack_raw_lines.pop().unwrap().len().div_ceil(4);
    let mut stacks: Vec<Vec<Crate>> = vec![Vec::new(); stacks_count];
    for line in stack_raw_lines.iter().rev() {
        for i in 0..stacks_count {
            match line.chars().nth(i * 4 + 1) {
                None | Some(' ') => continue,
                Some(ch) => stacks[i].push(ch),
            }
        }
    }
    debug!("Created {} stacks: {:?}", stacks_count, stacks);
    stacks
}

fn parse_transfer(raw: &str) -> ErrWrapper<TransferOrder> {
    let captures = RE_TRANSFER.captures(raw).ok_or(simple_error!("Unable to parse row"))?;
    Ok(TransferOrder {
        amount: captures.get(1).ok_or(simple_error!("No amount in transfer order"))?.as_str().parse()?,
        source: captures.get(2).ok_or(simple_error!("No source in transfer order"))?.as_str().parse()?,
        destination: captures.get(3).ok_or(simple_error!("No destination in transfer order"))?.as_str().parse()?,
    })
}

fn print_stacks(stacks: &Vec<Vec<Crate>>) {
    let max_stack_height = stacks.iter().map(|st| st.len()).max().unwrap();
    for i in (0..max_stack_height).rev() {
        let mut line = "".to_owned();
        for stack in stacks {
            let addition = match stack.get(i) {
                Some(cr) => format!("[{}] ", cr),
                None => "    ".to_owned(),
            };
            line += &addition;
        }
        info!("{}", line);
    }
    let footer: String = (1..=stacks.len()).map(|i| i.to_string()).intersperse("   ".to_owned()).collect();
    info!(" {}", footer); // The extra space is intentional for alignment
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_crates_one() {
        let lines = vec![
            "[S]".to_owned(),
            "[T]".to_owned(),
            " 1 ".to_owned(),
        ];
        let expected = vec![
            vec!['T', 'S']
        ];
        assert_eq!(expected, parse_crates(lines));
    }

    #[test]
    fn parse_crates_multi() {
        let lines = vec![
            "[S] [B]".to_owned(),
            "[T] [A]".to_owned(),
            " 1   2 ".to_owned(),
        ];
        let expected = vec![
            vec!['T', 'S'],
            vec!['A', 'B'],
        ];
        assert_eq!(expected, parse_crates(lines));
    }

    #[test]
    fn parse_crates_diff_heights() {
        let lines = vec![
            "[S]    ".to_owned(),
            "[T] [A]".to_owned(),
            " 1   2 ".to_owned(),
        ];
        let expected = vec![
            vec!['T', 'S'],
            vec!['A'],
        ];
        assert_eq!(expected, parse_crates(lines));
    }

    #[test]
    fn test_parse_transfer() {
        let expected = TransferOrder {
            amount: 11,
            source: 22,
            destination: 33,
        };
        assert_eq!(expected, parse_transfer("move 11 from 22 to 33").unwrap())
    }
}
