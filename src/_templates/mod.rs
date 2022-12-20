use crate::prelude::*;

#[derive(Debug)]
#[derive(clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

pub fn run(args: Args) -> RootErr {
    debug!("Args: {:?}", args);
    let line_src = files::read_lines(args.file)?;

    // TODO

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // TODO
    }
}
