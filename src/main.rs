use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "Help in administation")]
struct Cli {
    #[argh(subcommand)]
    sub: SubcommandsEnum
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubcommandsEnum {
    One(Generator),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate", description = "generate something")]
struct Generator {
    #[argh(subcommand)]
    sub: GeneratorSubcommands
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum GeneratorSubcommands {
    One(Certnames),
    Two(Emails)
}


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "certnames", description = "generate certnames")]
struct Certnames {
    #[argh(option, description = "change input file path")]
    from: String,
    #[argh(option, description = "change output file path")]
    to: String,
    #[argh(switch, short = 'd', description = "disable generation")]
    disable_generation: bool,
    #[argh(switch, short = 'p', description = "enable printing")]
    printing: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "emails", description = "generate emails")]
struct Emails {
    #[argh(option, description = "change input file path")]
    from: String,
    #[argh(option, description = "change output file path")]
    to: String,
    #[argh(option, default = "String::from(\"@telesales-service.org\")", description = "change email postfix")]
    postfix: String,
    #[argh(switch, short = 'd', description = "disable generation")]
    disable_generation: bool,
    #[argh(switch, short = 'p', description = "enable printing")]
    printing: bool,
}

fn main() {
    let cli: Cli = argh::from_env();
    println!("{:?}", cli)
}
