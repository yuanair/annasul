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
//!
//! # app
//! ```
//! // install rustup
//! #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
//! annasul::app::apps::desktop::rustup::install().unwrap();
//! ```
//!

#[cfg(any(doc, feature = "app"))]
pub mod app;
mod os_impl;

mod marco;
