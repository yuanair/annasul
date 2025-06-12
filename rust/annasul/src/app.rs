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
//! application operation
//!

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Pointer, Write};
use std::path::Path;

#[cfg(any(doc, feature = "app-apps"))]
pub mod apps;

#[derive(Debug)]
pub enum Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error => f.write_str(""),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
pub trait App<'a> {
    fn name() -> Cow<'a, str>;
    fn version() -> Result<Cow<'a, str>>;
    fn authors() -> Result<impl Iterator<Item = Cow<'a, str>>>;
    fn home_page() -> Result<Cow<'a, str>>;
    fn license() -> Result<Option<Cow<'a, str>>>;
    fn license_file() -> Result<Option<Cow<'a, Path>>>;
    fn description() -> Result<Cow<'a, str>>;
    fn documentation() -> Result<Cow<'a, str>>;
}
