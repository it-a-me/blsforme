// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

pub mod bootloader;
pub mod os_release;

/// Core configuration for boot management
#[derive(Debug)]
pub struct Configuration {
    /// Root of all operations
    pub root: Root,

    /// Where we can find `sysfs` `proc` etc
    pub vfs: PathBuf,
}

/// Wrap a root into a strong type to avoid confusion
#[derive(Debug)]
pub enum Root {
    /// Native installation
    Native(PathBuf),

    /// Image generation
    Image(PathBuf),
}

impl Root {
    /// When we don't need the type of the root..
    pub fn path(&self) -> &PathBuf {
        match self {
            Root::Native(p) => p,
            Root::Image(p) => p,
        }
    }
}
