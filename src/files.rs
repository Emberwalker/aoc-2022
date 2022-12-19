use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// see https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

// Used for testing read_lines consumers
#[cfg(test)]
macro_rules! read_lines_result_vec {
    ( $( $ex:expr ),* ) => {
        vec![
            $(Ok($ex.into())),*
        ].into_iter()
    }
}
