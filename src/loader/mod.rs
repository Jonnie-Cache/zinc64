// This file is part of zinc64.
// Copyright (c) 2016-2018 Sebastian Jastrzebski. All rights reserved.
// Licensed under the GPLv3. See LICENSE file in the project root for full license text.

mod bin;
mod crt;
//mod hex;
mod loaders;
mod p00;
mod prg;
mod tap;

use std::io;
use std::path::Path;

use system::{AutostartMethod, Image};

pub use self::bin::BinLoader;
pub use self::loaders::Loaders;

pub trait Loader {
    fn autostart(&self, path: &Path) -> Result<AutostartMethod, io::Error>;
    fn load(&self, path: &Path) -> Result<Box<Image>, io::Error>;
}
