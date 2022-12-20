#![feature(type_alias_impl_trait)]

use clap::{Parser, Subcommand};

#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

pub type ErrWrapper<T> = Result<T, Box<dyn std::error::Error>>;
pub type RootErr = Result<(), Box<dyn std::error::Error>>;

#[macro_use]
mod files;

mod day1;
mod day2;
mod day3;
mod day4;

mod prelude {
    pub(crate) use crate::files;
    pub(crate) use crate::{ErrWrapper, RootErr};
}

macro_rules! subcommands {
    ( $( $name:ident, $args:ty, $fn:expr ),+ ) => {
        #[derive(Debug)]
        #[derive(Subcommand)]
        enum Subcommands {
            $(
                // Command
                $name($args),
            )*
        }

        fn match_commands(args: Cli) {
            match args.command {
                $(
                    Subcommands::$name(args) => {
                        if let Err(err) = $fn(args) {
                            error!("$name error: {}", err);
                            std::process::exit(-1);
                        }
                    }
                )*
            }
        }
    };
}

// Add subcommands here!
// Enum value name, Args type, fn(Args) -> ErrWrapper
subcommands![
    Day1, day1::Args, day1::run,
    Day2a, day2::Args, day2::run_a,
    Day2b, day2::Args, day2::run_b,
    Day3, day3::Args, day3::run,
    Day4, day4::Args, day4::run
];

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info").default_write_style_or("auto"));
    let cli = Cli::parse();
    match_commands(cli);
}
