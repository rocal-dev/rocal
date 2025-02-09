use clap::{builder::Str, command, Arg, Command, Id};

use crate::{build::build, init::init};

pub fn run() {
    let matches = command!()
        .subcommand(
            Command::new(Subcommand::New)
                .about("Create a new rocal app")
                .arg(
                    Arg::new(InitCommandArg::Name)
                        .short('n')
                        .long("name")
                        .required(true)
                        .help("Set the resulting package name"),
                ),
        )
        .subcommand(Command::new(Subcommand::Build).about("Build a rocal app"))
        .about("A tool to create and build a rocal app.")
        .get_matches();

    match matches.subcommand() {
        Some((name, arg_matches)) => {
            if name == Subcommand::New.as_str() {
                if let Some(name) = arg_matches.get_one::<String>(InitCommandArg::Name.as_str()) {
                    init(name);
                }
            } else if name == Subcommand::Build.as_str() {
                build();
            }
        }
        None => (),
    }
}

enum Subcommand {
    New,
    Build,
}

enum InitCommandArg {
    Name,
}

impl Into<Str> for Subcommand {
    fn into(self) -> Str {
        self.as_str().into()
    }
}

impl Subcommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Subcommand::New => "new",
            Subcommand::Build => "build",
        }
    }
}

impl Into<Id> for InitCommandArg {
    fn into(self) -> Id {
        self.as_str().into()
    }
}

impl InitCommandArg {
    pub fn as_str(self) -> &'static str {
        match self {
            InitCommandArg::Name => "name",
        }
    }
}
