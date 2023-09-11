use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "Help in administation")]
pub struct Cli {
    #[argh(subcommand)]
    pub sub: SubcommandsEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubcommandsEnum {
    One(Generator),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate", description = "generate something")]
pub struct Generator {
    #[argh(subcommand)]
    pub sub: GeneratorSubcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum GeneratorSubcommands {
    One(Certnames),
    Two(Emails),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "certnames", description = "generate certnames")]
pub struct Certnames {
    #[argh(option, description = "change input file path")]
    pub from: String,
    #[argh(option, description = "change output file path")]
    pub to: String,
    #[argh(switch, short = 'd', description = "disable generation")]
    pub disable_generation: bool,
    #[argh(switch, short = 'p', description = "enable printing")]
    pub printing: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "emails", description = "generate emails")]
pub struct Emails {
    #[argh(option, description = "change input file path")]
    pub from: String,
    #[argh(option, description = "change output file path")]
    pub to: String,
    #[argh(
        option,
        default = "String::from(\"@telesales-service.org\")",
        description = "change email postfix"
    )]
    pub postfix: String,
    #[argh(switch, short = 'd', description = "disable generation")]
    pub disable_generation: bool,
    #[argh(switch, short = 'p', description = "enable printing")]
    pub printing: bool,
}
