use crate::cli::*;
use crate::helping_hand::*;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{stdin, IsTerminal};
use std::path::Path;

mod cli;
mod helping_hand;

fn main() {
    let is_pipe_mode = !stdin().is_terminal();
    let cli: Cli = argh::from_env();
    match cli.sub {
        SubcommandsEnum::One(ref generator) => match generator.sub {
            GeneratorSubcommands::One(ref certnames) => println!("{:?}", certnames),
            GeneratorSubcommands::Two(ref emails) => emails_proc(
                &emails.from,
                &emails.to,
                &emails.postfix,
                emails.disable_generation,
                emails.printing,
            ),
        },
    }
}


fn _add_postfix(root: String, postfix: String) -> String {
    return format!("{}{}", root, postfix);
}

fn _certnames(_from: String, _to: String, _disable_generation: bool, _printing: bool) {}

fn emails_proc(
    from: &String,
    to: &String,
    postfix: &String,
    disable_generation: bool,
    printing: bool,
) {
    let mut r = Vec::new();
    let path_to_inputfile = Path::new(from);
    let inputfile = read_to_string(path_to_inputfile)
        .expect(&format!("Something wrong with {}", &path_to_inputfile.display())[..]);
    for line in inputfile.lines() {
        r.push(line.to_string().to_lowercase())
    }
    let uniques: HashSet<_> = r.iter().cloned().collect();
    for unique in uniques {
        println!("{:?}", to_cert_name(transliterate(&unique)));
    }
}
