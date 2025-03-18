use clap::{builder::Str, command, Arg, Command, Id};

use crate::commands::{
    build::build, init::init, login::login, password, publish::publish, register::register,
    subscribe::subscribe, unsubscribe::unsubscribe,
};

/*
$ rocal register
Email:
Password:
Workspace:

$ rocal login
EMAIL:
PASSWORD:

// $ rocal workspace new -n <workspace>
// $ rocal workspace change -n <workspace>
// $ rocal workspace list
// - * A (default)
// - B

$ rocal subscribe
PLAN 1: $10 - versioning, rollback, caching
PLAN 2: $20 - sync server

$ rocal unsubscribe

$ rocal token list
5096d13e-7ed2-4415-9fd6-fe1e52ccedd9
16be024b-ef4c-4308-a312-1c001df9b973

$ rocal token new
df20f838-ed31-43e8-97fd-3d5db69cf176

$ rocal token revoke <token>
$ rocal token set <token>

$ rocal build --dev
$ rocal build --release

$ rocal publish
if there is not token, it raises an error

$ rocal password reset
Email:
 */

pub async fn run() {
    let matches = command!()
        .subcommand(
            Command::new(Subcommand::Register)
                .about("Create a new account in Rocal platform")
        )
        .subcommand(
            Command::new(Subcommand::Login)
                .about("Login to Rocal platform")
        )
        .subcommand(
            Command::new(Subcommand::Subscribe)
                .about("Subscribe Rocal platform to publish a Rocal app")
        )
        .subcommand(
            Command::new(Subcommand::Unsubscribe)
                .about("Unsubscribe Rocal platform which leads to revoke tokens and shut your hosting app down")
        )
        .subcommand(
            Command::new(Subcommand::Token)
                .about("Manipulate token that is required to publish a Rocal app")
                .subcommand(Command::new(TokenSubcommand::New).about("Create a new token"))
                .subcommand(Command::new(TokenSubcommand::List).about("List tokens you created"))
                .subcommand(Command::new(TokenSubcommand::Revoke).about("Revoke a token"))
                .subcommand(Command::new(TokenSubcommand::Set).about("Set a token"))
        )
        .subcommand(
            Command::new(Subcommand::New)
                .about("Create a new Rocal app")
                .arg(
                    Arg::new(InitCommandArg::Name)
                        .short('n')
                        .long("name")
                        .required(true)
                        .help("Set the resulting package name"),
                ),
        )
        .subcommand(Command::new(Subcommand::Build).about("Build a Rocal app"))
        .subcommand(Command::new(Subcommand::Publish).about("Publish a Rocal app"))
        .subcommand(
            Command::new(Subcommand::Password)
                .about("Password settings")
                .subcommand(Command::new(PasswordSubcommand::Reset).about("Reset your password"))
        )
        .about("A tool to create and build a Rocal app.")
        .get_matches();

    match matches.subcommand() {
        Some((name, arg_matches)) => {
            if name == Subcommand::New.as_str() {
                if let Some(name) = arg_matches.get_one::<String>(InitCommandArg::Name.as_str()) {
                    init(name);
                }
            } else if name == Subcommand::Build.as_str() {
                build();
            } else if name == Subcommand::Publish.as_str() {
                publish().await;
            } else if name == Subcommand::Register.as_str() {
                register().await;
            } else if name == Subcommand::Login.as_str() {
                login().await;
            } else if name == Subcommand::Password.as_str() {
                match arg_matches.subcommand() {
                    Some((name, _arg_matches)) => {
                        if name == PasswordSubcommand::Reset.as_str() {
                            password::reset().await;
                        }
                    }
                    None => (),
                }
            } else if name == Subcommand::Subscribe.as_str() {
                if let Err(err) = subscribe().await {
                    println!("Error: {}", err.to_string());
                }
            } else if name == Subcommand::Unsubscribe.as_str() {
                unsubscribe().await;
            }
        }
        None => (),
    }
}

enum Subcommand {
    Register,
    Login,
    Subscribe,
    Unsubscribe,
    Token,
    New,
    Build,
    Publish,
    Password,
}

enum TokenSubcommand {
    New,
    List,
    Revoke,
    Set,
}

enum PasswordSubcommand {
    Reset,
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
            Subcommand::Register => "register",
            Subcommand::Login => "login",
            Subcommand::Subscribe => "subscribe",
            Subcommand::Unsubscribe => "unsubscribe",
            Subcommand::Token => "token",
            Subcommand::New => "new",
            Subcommand::Build => "build",
            Subcommand::Publish => "publish",
            Subcommand::Password => "password",
        }
    }
}

impl Into<Str> for TokenSubcommand {
    fn into(self) -> Str {
        self.as_str().into()
    }
}

impl TokenSubcommand {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenSubcommand::New => "new",
            TokenSubcommand::List => "list",
            TokenSubcommand::Revoke => "revoke",
            TokenSubcommand::Set => "set",
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

impl Into<Str> for PasswordSubcommand {
    fn into(self) -> Str {
        self.as_str().into()
    }
}

impl PasswordSubcommand {
    pub fn as_str(self) -> &'static str {
        match self {
            PasswordSubcommand::Reset => "reset",
        }
    }
}
