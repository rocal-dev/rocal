use clap::{builder::Str, command, Arg, Command, Id};

use crate::commands::{
    build::build, init::init, login::login, migrate, password, publish::publish,
    register::register, subscribe::subscribe, sync_servers, unsubscribe::unsubscribe,
};

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
        .subcommand(
            Command::new(Subcommand::Run)
                .about("Run a Rocal app on your local")
                .arg(
                    Arg::new(RunCommandArg::Port)
                        .short('p')
                        .long("port")
                        .required(false)
                        .help("Set port where you want to serve an app")
                )
        )
        .subcommand(Command::new(Subcommand::Publish).about("Publish a Rocal app"))
        .subcommand(
            Command::new(Subcommand::Password)
                .about("Password settings")
                .arg_required_else_help(true)
                .subcommand(Command::new(PasswordSubcommand::Reset).about("Reset your password"))
        )
        .subcommand(
            Command::new(Subcommand::SyncServers)
                .about("Manage sync servers")
                .arg_required_else_help(true)
                .subcommand(Command::new(SyncServersSubcommand::List).about("List available sync servers and show app_id"))
        )
        .subcommand(
            Command::new(Subcommand::Migrate)
                .about("Manage migrations")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(MigrateSubcommand::Add)
                        .about("Add a new migration file. e.g. db/migrations/<timestamp>-<name>.sql")
                        .arg(Arg::new("name").required(true))
                )
        )
        .about("A tool to create and build a Rocal app.")
        .arg_required_else_help(true)
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
            } else if name == Subcommand::Migrate.as_str() {
                match arg_matches.subcommand() {
                    Some((name, arg_matches)) => {
                        if name == MigrateSubcommand::Add.as_str() {
                            let name = arg_matches
                                .get_one::<String>("name")
                                .expect("required argument");
                            migrate::add(&name);
                        }
                    }
                    None => (),
                }
            } else if name == Subcommand::SyncServers.as_str() {
                match arg_matches.subcommand() {
                    Some((name, _arg_matches)) => {
                        if name == SyncServersSubcommand::List.as_str() {
                            sync_servers::list().await;
                        }
                    }
                    None => (),
                }
            } else if name == Subcommand::Run.as_str() {
                build();
                if let Some(port) = arg_matches.get_one::<String>(RunCommandArg::Port.as_str()) {
                    rocal_dev_server::run(Some(&port));
                } else {
                    rocal_dev_server::run(None);
                }
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
    New,
    Build,
    Publish,
    Password,
    SyncServers,
    Run,
    Migrate,
}

enum PasswordSubcommand {
    Reset,
}

enum InitCommandArg {
    Name,
}

enum SyncServersSubcommand {
    List,
}

enum RunCommandArg {
    Port,
}

enum MigrateSubcommand {
    Add,
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
            Subcommand::New => "new",
            Subcommand::Build => "build",
            Subcommand::Publish => "publish",
            Subcommand::Password => "password",
            Subcommand::SyncServers => "sync-servers",
            Subcommand::Run => "run",
            Subcommand::Migrate => "migrate",
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

impl Into<Str> for SyncServersSubcommand {
    fn into(self) -> Str {
        self.as_str().into()
    }
}

impl SyncServersSubcommand {
    pub fn as_str(self) -> &'static str {
        match self {
            SyncServersSubcommand::List => "ls",
        }
    }
}

impl Into<Str> for MigrateSubcommand {
    fn into(self) -> Str {
        self.as_str().into()
    }
}

impl MigrateSubcommand {
    pub fn as_str(self) -> &'static str {
        match self {
            MigrateSubcommand::Add => "add",
        }
    }
}

impl Into<Id> for RunCommandArg {
    fn into(self) -> Id {
        self.as_str().into()
    }
}

impl RunCommandArg {
    pub fn as_str(self) -> &'static str {
        match self {
            RunCommandArg::Port => "port",
        }
    }
}
