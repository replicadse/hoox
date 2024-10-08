use std::str::FromStr;

use anyhow::Result;
use clap::{
    Arg,
    ArgAction,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Privilege {
    Normal,
    Experimental,
}

#[derive(Debug)]
pub struct CallArgs {
    pub privileges: Privilege,
    pub command: Command,
}

impl CallArgs {
    pub fn validate(&self) -> Result<()> {
        if self.privileges == Privilege::Experimental {
            return Ok(());
        }

        match &self.command {
            // | Command::Experimental { .. } => Err(Error::ExperimentalCommand("watch".to_owned()))?,
            | _ => (),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ManualFormat {
    Manpages,
    Markdown,
}

#[derive(Debug)]
pub enum InitTemplate {
    Rust,
}

#[derive(Debug)]
pub enum Command {
    Manual {
        path: String,
        format: ManualFormat,
    },
    Autocomplete {
        path: String,
        shell: clap_complete::Shell,
    },

    Init {
        template: InitTemplate,
    },
    Run {
        hook: String,
        args: Vec<String>,
        ignore_missing: bool,
    },
}

pub struct ClapArgumentLoader {}

impl ClapArgumentLoader {
    pub fn root_command() -> clap::Command {
        clap::Command::new("hoox")
            .version(env!("CARGO_PKG_VERSION"))
            .about("hoox - git hooks done right")
            .author("Alexander Weber <aw@voidpointergroup.com>")
            .propagate_version(true)
            .subcommand_required(true)
            .args([Arg::new("experimental")
                .short('e')
                .long("experimental")
                .help("Enables experimental features.")
                .num_args(0)])
            .subcommand(
                clap::Command::new("man")
                    .about("Renders the manual.")
                    .arg(clap::Arg::new("out").short('o').long("out").required(true))
                    .arg(
                        clap::Arg::new("format")
                            .short('f')
                            .long("format")
                            .value_parser(["manpages", "markdown"])
                            .required(true),
                    ),
            )
            .subcommand(
                clap::Command::new("autocomplete")
                    .about("Renders shell completion scripts.")
                    .arg(clap::Arg::new("out").short('o').long("out").required(true))
                    .arg(
                        clap::Arg::new("shell")
                            .short('s')
                            .long("shell")
                            .value_parser(["bash", "zsh", "fish", "elvish", "powershell"])
                            .required(true),
                    ),
            )
            .subcommand(clap::Command::new("init").about("Initialize repository hooks.").arg(
                clap::Arg::new("template").short('t').long("template").value_parser(["rust"]).default_value("rust"),
            ))
            .subcommand(
                clap::Command::new("run")
                    .about("Run a hook.")
                    .arg(clap::Arg::new("hook").required(true).index(1))
                    .arg(clap::Arg::new("args").required(false).action(ArgAction::Append).index(2))
                    .arg(
                        clap::Arg::new("ignore-missing")
                            .long("ignore-missing")
                            .required(false)
                            .action(ArgAction::SetTrue),
                    ),
            )
    }

    pub fn load() -> Result<CallArgs> {
        let command = Self::root_command().get_matches();

        let privileges = if command.get_flag("experimental") {
            Privilege::Experimental
        } else {
            Privilege::Normal
        };

        let cmd = if let Some(subc) = command.subcommand_matches("man") {
            Command::Manual {
                path: subc.get_one::<String>("out").unwrap().into(),
                format: match subc.get_one::<String>("format").unwrap().as_str() {
                    | "manpages" => ManualFormat::Manpages,
                    | "markdown" => ManualFormat::Markdown,
                    | _ => return Err(anyhow::anyhow!("unknown format")),
                },
            }
        } else if let Some(subc) = command.subcommand_matches("autocomplete") {
            Command::Autocomplete {
                path: subc.get_one::<String>("out").unwrap().into(),
                shell: clap_complete::Shell::from_str(subc.get_one::<String>("shell").unwrap().as_str()).unwrap(),
            }
        } else if let Some(subc) = command.subcommand_matches("init") {
            Command::Init {
                template: match subc.get_one::<String>("template").unwrap().as_str() {
                    | "rust" => InitTemplate::Rust,
                    | _ => return Err(anyhow::anyhow!("unknown init template")),
                },
            }
        } else if let Some(subc) = command.subcommand_matches("run") {
            Command::Run {
                hook: subc.get_one::<String>("hook").unwrap().to_owned(),
                args: match subc.get_many::<String>("args") {
                    | Some(v) => v.map(|v| v.to_string()).collect::<Vec<String>>(),
                    | None => vec![],
                },
                ignore_missing: subc.get_flag("ignore-missing"),
            }
        } else {
            return Err(anyhow::anyhow!("unknown command"));
        };

        let callargs = CallArgs {
            privileges,
            command: cmd,
        };

        callargs.validate()?;
        Ok(callargs)
    }
}
