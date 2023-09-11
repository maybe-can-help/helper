use crate::cli::*;
use std::fs::File;

mod cli;

fn main() {
    let cli: Cli = argh::from_env();
    match cli.sub {
        SubcommandsEnum::One(ref generator) => match generator.sub {
            GeneratorSubcommands::One(ref certnames) => println!("{:?}", certnames),
            GeneratorSubcommands::Two(ref emails) => println!("{:?}", emails),
        },
    }
}

fn add_postfix(root: String, postfix: String) -> String {
    return format!("{}{}", root, postfix);
}

fn _certnames(_from: String, _to: String, _disable_generation: bool, _printing: bool) {}

fn _emails(
    from: String,
    _to: String,
    _postfix: String,
    _disable_generation: bool,
    _printing: bool,
) {
    let _input_file = File::open(from);
}
