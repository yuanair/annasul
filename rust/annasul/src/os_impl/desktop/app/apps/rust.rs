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
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Pointer, Write};
use std::io::{Read, Stderr, Stdin, Stdout};
use std::os::fd::{AsFd, AsRawFd};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, Command};

#[derive(Debug)]
pub struct Rustup {
    home_path: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    Unsupported(String),
    IOError(std::io::Error),
    InnerError(String),
    Failed{exit_status: ExitStatus, stdin: Stdin, stdout: Stdout, stderr: Stderr},
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported(info) => f.write_fmt(format_args!("Unsupported: {}", info)),
            Error::IOError(e) => f.write_fmt(format_args!("IO error: {}", e)),
            Error::InnerError(info) => f.write_fmt(format_args!("Inner error: {}", info)),
            Error::Failed{exit_status, stdin, stdout, stderr} => 
                f.write_fmt(format_args!("Failed:\n - exit status: {}\n - stdin:\n{}\n\n - stdout:\n{}\n\n - stderr:\n{}", exit_status, OsStr::new(&stdin.bytes()).to_string_lossy(), stdout., stderr))
            ,
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Unsupported(_) => None,
            Error::IOError(e) => Some(e),
            Error::InnerError(_) => None,
            Error::Failed{..} => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Default, Debug)]
pub enum Toolchain {
    #[default]
    Stable,
    Beta,
    Nightly,
    None,
}
#[derive(Default, Debug)]
pub enum HostTriple {
    #[default]
    Host,
    /// e.g. x86_64-unknown-linux-gnu
    Target(String),
}
#[derive(Default, Debug)]
pub enum Profile {
    Minimal,
    #[default]
    Default,
    Complete,
}
#[derive(Debug)]
pub struct InstallCustomInfo {
    default_host_triple: HostTriple,
    default_toolchain: Toolchain,
    profile: Profile,
    modify_path_variable: bool,
}
#[derive(Default, Debug)]
pub enum InstallInfo {
    #[default]
    Default,
    Custom(InstallCustomInfo),
}
impl Default for InstallCustomInfo {
    fn default() -> Self {
        Self {
            modify_path_variable: true,
            ..Default::default()
        }
    }
}
impl<'a> crate::app::App<'a> for Rustup {
    type Error = Error;
    type InstallInfo = InstallInfo;
    type ReinstallInfo = ();
    type RemoveInfo = ();
    type UpdateInfo = ();
    fn name(&self) -> Cow<'a, str> {
        Cow::Borrowed("rustup")
    }

    async fn license(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn license_file(&self) -> Result<Option<Cow<'a, Path>>> {
        todo!()
    }

    async fn description(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn documentation(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn home_page(&self) -> Result<Option<Cow<'a, str>>> {
        Ok(Some(Cow::Borrowed("https://rustup.rs/")))
    }

    async fn home_path(&self) -> Result<Option<Cow<'a, Path>>> {
        todo!()
    }

    async fn bin_path(&self) -> Result<Option<Cow<'a, Path>>> {
        todo!()
    }

    async fn version(&self) -> Result<Cow<'a, str>> {
        todo!()
    }

    async fn install(&self, info: Self::InstallInfo) -> Result<()> {
        // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        if cfg!(unix) {
            let mut command = Command::new("curl")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .args([
                    "--proto",
                    "'=https'",
                    "--tlsv1.2",
                    "-sSf",
                    "https://sh.rustup.rs",
                ])
                .spawn()
                .map_err(Error::IOError)?;

            let (mut stdin, stdout, stderr) = (
                command
                    .stdin
                    .take()
                    .ok_or(Error::InnerError("stdin is not available".to_string()))?,
                command
                    .stdout
                    .as_ref()
                    .ok_or(Error::InnerError("stdout is not available".to_string()))?,
                command
                    .stderr
                    .as_ref()
                    .ok_or(Error::InnerError("stderr is not available".to_string()))?,
            );

            match info {
                InstallInfo::Default => {
                    Pin::new(&mut stdin)
                        .write_all("1\n".as_bytes())
                        .await
                        .map_err(Error::IOError)?;
                }
                InstallInfo::Custom(InstallCustomInfo {
                    default_host_triple,
                    default_toolchain,
                    profile,
                    modify_path_variable,
                }) => {
                    Pin::new(&mut stdin)
                        .write_all(
                            format!(
                                "2\n{}\n{}\n{}\n{}\n1",
                                default_host_triple,
                                default_toolchain,
                                profile,
                                if modify_path_variable { "Y" } else { "n" }
                            )
                            .as_bytes(),
                        )
                        .await
                        .map_err(Error::IOError)?;
                }
            }

            let mut output_buf = Vec::new();

            let write_handle = tokio::spawn(async move {
                stdin.write_all(b"hello async world").await?;
                stdin.shutdown().await?;
                Ok::<_, std::io::Error>(())
            });

            let read_handle = tokio::spawn(async move {
                stdout.read_to_end(&mut output_buf).await
            });

            let (write_result, read_result) = tokio::join!(write_handle, read_handle);
            write_result??;
            read_result??;
            
            stdin.shutdown().await.map_err(Error::IOError)?;
            let exit_status = command.wait().await.map_err(Error::IOError)?;
            if exit_status.success() {
                Ok(())
            } else {
                Err(Error::Failed {exit_status ,stdout: stdout., stderr:stderr})
            }
        } else if cfg!(windows) {
            todo!()
        } else {
            Err(Error::Unsupported(format!(
                "unsupported platform '{}'",
                std::env::consts::OS
            )))
        }
    }

    async fn reinstall(&self, info: Self::ReinstallInfo) -> Result<()> {
        todo!()
    }

    async fn remove(&self, info: Self::RemoveInfo) -> Result<()> {
        todo!()
    }

    async fn update(&self, info: Self::UpdateInfo) -> Result<()> {
        todo!()
    }
}
