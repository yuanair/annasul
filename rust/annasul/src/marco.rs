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

#[macro_export]
macro_rules! mod_all_os {
    {$vis:vis} => {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        $vis mod desktop;
        #[cfg(any(target_os = "android", target_os = "ios"))]
        $vis mod mobile;
        #[cfg(target_os = "linux")]
        $vis mod linux;
        #[cfg(target_os = "macos")]
        $vis mod macos;
        #[cfg(target_os = "windows")]
        $vis mod windows;
        #[cfg(target_os = "ios")]
        $vis mod ios;
        #[cfg(target_os = "android")]
        $vis mod android;
    };
}
#[macro_export]
macro_rules! use_all_os {
    {$vis:vis, :$sub:path: $($paths:path),*} => {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        $vis use self::desktop::$sub::{$($path,)*};
        #[cfg(any(target_os = "android", target_os = "ios"))]
        $vis use self::mobile::$sub::{$($path,)*};
        #[cfg(target_os = "linux")]
        $vis use self::linux::$sub::{$($path,)*};
        #[cfg(target_os = "macos")]
        $vis use self::macos::$sub::{$($path,)*};
        #[cfg(target_os = "windows")]
        $vis use self::windows::$sub::{$($path,)*};
        #[cfg(target_os = "ios")]
        $vis use self::ios::$sub::{$($path,)*};
        #[cfg(target_os = "android")]
        $vis use self::android::$sub::{$($path,)*};
    };
    {$vis:vis, $base:path: $sub:path: $($paths:path),*} => {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        $vis use $base::desktop::$sub::{$($path,)*};
        #[cfg(any(target_os = "android", target_os = "ios"))]
        $vis use $base::mobile::$sub::{$($path,)*};
        #[cfg(target_os = "linux")]
        $vis use $base::linux::$sub::{$($path,)*};
        #[cfg(target_os = "macos")]
        $vis use $base::macos::$sub::{$($path,)*};
        #[cfg(target_os = "windows")]
        $vis use $base::windows::$sub::{$($path,)*};
        #[cfg(target_os = "ios")]
        $vis use $base::ios::$sub::{$($path,)*};
        #[cfg(target_os = "android")]
        $vis use $base::android::$sub::{$($path,)*};
    };
    {$vis:vis, $base:path: $($paths:path),*} => {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        $vis use $base::desktop::{$($path,)*};
        #[cfg(any(target_os = "android", target_os = "ios"))]
        $vis use $base::mobile::{$($path,)*};
        #[cfg(target_os = "linux")]
        $vis use $base::linux::{$($path,)*};
        #[cfg(target_os = "macos")]
        $vis use $base::macos::{$($path,)*};
        #[cfg(target_os = "windows")]
        $vis use $base::windows::{$($path,)*};
        #[cfg(target_os = "ios")]
        $vis use $base::ios::{$($path,)*};
        #[cfg(target_os = "android")]
        $vis use $base::android::{$($path,)*};
    };
    {$vis:vis, $($paths:path),*} => {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        $vis use self::desktop::{$($path,)*};
        #[cfg(any(target_os = "android", target_os = "ios"))]
        $vis use self::mobile::{$($path,)*};
        #[cfg(target_os = "linux")]
        $vis use self::linux::{$($path,)*};
        #[cfg(target_os = "macos")]
        $vis use self::macos::{$($path,)*};
        #[cfg(target_os = "windows")]
        $vis use self::windows::{$($path,)*};
        #[cfg(target_os = "ios")]
        $vis use self::ios::{$($path,)*};
        #[cfg(target_os = "android")]
        $vis use self::android::{$($path,)*};
    }
}
