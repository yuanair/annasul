// Copyright (c) 2025 air (https://yuanair.github.io).
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//!
//! # commands:
//! > > ![feature] auto-completion (install|reinstall|remove) script
//! > > ```shell
//! > > $ abuild auto-complete install bash
//! > > the auto-completion script for bash was installed in '/etc/bash_completion.d/abuild' successfully.
//! > > $ abuild auto-complete reinstall zsh
//! > > the auto-completion script for zsh was installed in '/usr/local/share/zsh/site-functions/_abuild' successfully.
//! > > $ abuild auto-complete remove bash
//! > > the auto-completion script for bash was removed from '/etc/bash_completion.d/abuild' successfully.
//! > > ```
//! >
//! > > ![feature] init/create/remove (workspace|project|profile)
//! > > + ![note] init profile: unsupported yet.
//! > > + ![note] init/create: The workspace directory must be empty.
//! > > + ![note] create/remove profile: The current folder must be a (workspace|project), or the (-w|-j) option must be provided.
//! > > ```shell
//! > > $ abuild init
//! > > workspace '<current_directory>' was initialized successfully.
//! > > $ abuild create
//! > > workspace '<workspace_name>' was created successfully.
//! > > $ abuild remove
//! > > workspace '<workspace_name>' was removed successfully.
//! > > ```
//! >
//! > > ![feature] undo/redo
//! > > ```shell
//! > > $ abuild undo
//! > > the last operation is '<last_operation>'
//! > > ... # output of the undo operation
//! > > $ abuild redo
//! > > the last operation is '<last_operation>'
//! > > ... # output of the redo operation
//! > > ```
//! >
//! > > ![feature] build/clean (workspace|project|profile)
//! > > ```shell
//! > > $ abuild build
//! > > building...
//! > > ... # output of the build process
//! > > building finished.
//! > > $ abuild clean
//! > > cleaning...
//! > > ... # output of the clean process
//! > > cleaning finished.
//! > > ```
//! >
//! > > ![feature] run (workspace|project|profile)
//! > > ```shell
//! > > $ abuild run
//! > > ... # output of the build process (if not already built)
//! > > running...
//! > > ... # output of the run process
//! > > the program is exited with code '<exit_code>'.
//! > > ```
//! >
//! > > ![feature] rebuild = clean \& build (workspace|project|profile)
//! > > ```shell
//! > > $ abuild rebuild
//! > > ... # output of the clean process
//! > > ... # output of the build process
//! > > ```
//! >
//! > > ![feature] set/unset (workspace|project|profile)
//! > > ```shell
//! > > $ abuild set -w . config.author "your_name"
//! > > the workspace '<workspace_name>' config.author was set to "your_name".
//! > > $ abuild set -j my_project config.version "1.0.0"
//! > > the project '<project_name>' config.version was set to "1.0.0".
//! > > ```
//! >
//!
//! # Options:
//! - `-w, --workspace <workspace_path>`: set the workspace directory.
//! - `-j, --project <project_name>`: set the project name.
//! - `-p, --profile <profile_name>`: set the profile name.
//! - `-b, --binary <binary_name>`: set the binary name to build or run.
//! - `-a, --args <args>`: set the arguments to pass to the binary.
//!
//! [note]: https://img.shields.io/badge/note-orange.svg?color=ddbb00
//!
//! [bug]: https://img.shields.io/badge/bug-red.svg
//!
//! [feature]: https://img.shields.io/badge/feature-orange.svg
//!

use clap::{value_parser, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::aot::generate;
use clap_complete::Generator;
use colored::Colorize;
use io::Write;
use std::borrow::Cow;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    NoHomeDirError,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IO error: {}", e),
            Error::NoHomeDirError => write!(f, "No home directory found"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::NoHomeDirError => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    sub_command: SubCommand,
}

pub use clap_complete::Shell;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct ShellOptions {
    /// the shell to generate the auto-completion script for
    #[clap(value_parser=value_parser!(Shell))]
    shell: Shell,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum AutoCompleteSubCommand {
    /// install auto-completion script
    Install {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// reinstall auto-completion script
    Reinstall {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// remove auto-completion script
    Remove {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// output auto-completion script
    Output {
        #[clap(flatten)]
        shell: ShellOptions,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SubCommand {
    /// auto-completion script
    AutoComplete {
        #[clap(subcommand)]
        sub_command: AutoCompleteSubCommand,
    },
    /// initialize a new workspace or project
    Init {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// create a new workspace or project or profile
    Create {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// remove a workspace or project or profile
    Remove {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// undo the last command in workspace or project or profile
    Undo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// redo the last command in workspace or project or profile
    Redo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// build workspace or project or profile
    Build {
        /// the binary to build (default: build all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// clean workspace or project or profile
    Clean {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// run binary in a workspace or project or profile
    Run {
        /// the binary to run (default: run all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        /// the arguments to pass to the binary
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<OsString>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// rebuild workspace or project or profile
    Rebuild {
        /// the binary to rebuild (default: rebuild all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
}

/// Options for the scope of the command (workspace, project, profile)
#[derive(Default, Debug, Clone, PartialEq, Eq, Parser)]
pub struct ScopeOptions {
    /// set the workspace directory
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub workspace: Option<PathBuf>,
    /// set the project name
    #[clap(short = 'j', long, value_hint = ValueHint::DirPath)]
    pub project: Option<String>,
    /// set the profile name
    #[clap(short, long, value_hint = ValueHint::Unknown)]
    pub profile: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub fn generate_completion<G: Generator>(generator: G, bin_name: &str, buf: &mut dyn Write) {
    generate(generator, &mut Cli::command(), bin_name, buf);
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.sub_command().run()
    }
    pub fn sub_command(&self) -> &SubCommand {
        &self.sub_command
    }
    pub fn sub_command_mut(&mut self) -> &mut SubCommand {
        &mut self.sub_command
    }
}

impl ShellOptions {
    #[cfg(unix)]
    pub fn config_dir(&self) -> Result<Cow<'static, Path>> {
        match self.shell {
            Shell::Bash => Ok(Cow::Borrowed(Path::new("/etc/bash_completion.d"))),
            Shell::Zsh => Ok(Cow::Borrowed(Path::new(
                "/usr/local/share/zsh/site-functions",
            ))),
            Shell::Fish => Ok(Cow::Borrowed(Path::new(
                "/usr/share/fish/vendor_completions.d",
            ))),
            Shell::PowerShell => Ok(Cow::Borrowed(Path::new(
                "/usr/local/share/powershell/Modules/",
            ))),
            Shell::Elvish => Ok(Cow::Borrowed(Path::new("/usr/share/elvish/lib/"))),
            shell => panic!("unsupported shell: {}", shell),
        }
    }
    pub fn config_file_name(&self) -> Cow<'static, Path> {
        let app_name = crate::app_name();
        match self.shell {
            Shell::Bash => match app_name {
                Cow::Borrowed(app_name) => Cow::Borrowed(Path::new(app_name)),
                Cow::Owned(app_name) => Cow::Owned(PathBuf::from(app_name)),
            },
            Shell::Zsh => Cow::Owned(format!("_{}", app_name).into()),
            Shell::Fish => Cow::Owned(format!("{}.fish", app_name).into()),
            Shell::PowerShell => Cow::Owned(format!("{}.ps1", app_name).into()),
            Shell::Elvish => Cow::Owned(format!("_{}.elv", app_name).into()),
            shell => panic!("unsupported shell: {}", shell),
        }
    }
    pub fn config_file_path(&self) -> Result<Cow<'static, Path>> {
        let config_dir = self.config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).map_err(Error::IOError)?;
        }
        Ok(Cow::Owned(config_dir.join(self.config_file_name())))
    }
    pub fn show_installed_info(&self, config_file_path: Cow<Path>) {
        match self.shell {
            Shell::PowerShell => {
                println!(
                    "{}: Please run 'Import-Module \"{}\"' in powershell",
                    "INFO".bright_white(),
                    config_file_path.display()
                );
            }
            shell @ (Shell::Bash | Shell::Zsh | Shell::Fish) => {
                println!(
                    "{}: Please reset {1}, or run 'source \"{2}\"' in {1}",
                    "INFO".bright_white(),
                    shell,
                    config_file_path.display()
                );
            }
            Shell::Elvish => {
                println!(
                    "{}: Please reset {1}, or run '{1} \"{2}\"', \n\tFor more information, see https://github.com/zzamboni/elvish-completions.",
                    "INFO".bright_white(),
                    Shell::Elvish,
                    config_file_path.display(),
                );
            }
            shell => panic!("unsupported shell: {}", shell),
        }
    }
}

impl SubCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            SubCommand::AutoComplete { sub_command } => {
                let mut buffer = Vec::new();
                match sub_command {
                    AutoCompleteSubCommand::Output { shell } => {
                        generate_completion(shell.shell, &crate::app_name(), &mut buffer);
                        io::stdout().write_all(&buffer).map_err(Error::IOError)?;
                        Ok(())
                    }
                    AutoCompleteSubCommand::Install { shell } => {
                        let config_file_path = shell.config_file_path()?;
                        println!(
                            "the auto-completion script for {} will be installed in '{}'.",
                            shell.shell,
                            config_file_path.display()
                        );
                        {
                            let mut config_file = match std::fs::File::create_new(&config_file_path) {
                                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                                    println!("{}: the auto-completion script for {} is already installed.", "WARNING".bright_yellow(), shell.shell);
                                    return Ok(());
                                }
                                res => res,
                            }.map_err(Error::IOError)?;
                            generate_completion(shell.shell, &crate::app_name(), &mut config_file);
                        }
                        println!(
                            "the auto-completion script for {} was installed {}.",
                            shell.shell,
                            "successfully".bright_green()
                        );

                        shell.show_installed_info(config_file_path);

                        Ok(())
                    }
                    AutoCompleteSubCommand::Reinstall { shell } => {
                        let config_file_path = shell.config_file_path()?;
                        if config_file_path.exists() {
                            println!(
                                "the auto-completion script for {} will be reinstalled in '{}'.",
                                shell.shell,
                                config_file_path.display()
                            );
                            std::fs::remove_file(&config_file_path).map_err(Error::IOError)?;
                            {
                                let mut config_file = std::fs::File::create(&config_file_path)
                                    .map_err(Error::IOError)?;
                                generate_completion(
                                    shell.shell,
                                    &crate::app_name(),
                                    &mut config_file,
                                );
                            }
                            println!(
                                "the auto-completion script for {} was reinstalled {}.",
                                shell.shell,
                                "successfully".bright_green()
                            );

                            shell.show_installed_info(config_file_path);
                            Ok(())
                        } else {
                            println!(
                                "{}: the auto-completion script for {} was not installed in '{}'.",
                                "ERROR".bright_red(),
                                shell.shell,
                                config_file_path.display()
                            );
                            std::process::exit(1);
                        }
                    }
                    AutoCompleteSubCommand::Remove { shell } => {
                        let config_file_path = shell.config_file_path()?;
                        if config_file_path.exists() {
                            println!(
                                "the auto-completion script for {} will be removed from '{}'.",
                                shell.shell,
                                config_file_path.display()
                            );
                            std::fs::remove_file(&config_file_path).map_err(Error::IOError)?;
                            println!(
                                "the auto-completion script for {} was removed {}.",
                                shell.shell,
                                "successfully".bright_green()
                            );
                            Ok(())
                        } else {
                            println!(
                                "{}: the auto-completion script for {} was not installed in '{}'.",
                                "ERROR".bright_red(),
                                shell.shell,
                                config_file_path.display()
                            );
                            std::process::exit(1);
                        }
                    }
                }
            }
            SubCommand::Init { scope } => {
                todo!()
            }
            SubCommand::Create { scope } => {
                todo!()
            }
            SubCommand::Remove { scope } => {
                todo!()
            }
            SubCommand::Undo { scope } => {
                todo!()
            }
            SubCommand::Redo { scope } => {
                todo!()
            }
            SubCommand::Build { binary, scope } => {
                todo!()
            }
            SubCommand::Clean { scope } => {
                todo!()
            }
            SubCommand::Run {
                binary,
                args,
                scope,
            } => {
                todo!()
            }
            SubCommand::Rebuild { binary, scope } => {
                todo!()
            }
        }
    }
}
