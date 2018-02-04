/*
 * Copyright (c) 2016-2018 Sebastian Jastrzebski. All rights reserved.
 *
 * This file is part of zinc64.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs::File;
use std::io;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::Path;
use std::result::Result;
use std::str;

use byteorder::{LittleEndian, ReadBytesExt};
use system::{Autostart, AutostartMethod, C64, Image};
use system::autostart;

use super::Loader;

static HEADER_SIG: &'static str = "C64File";

struct Header {
    signature: [u8; 7],
    #[allow(dead_code)] reserved_1: u8,
    #[allow(dead_code)] filename: [u8; 16],
    #[allow(dead_code)] reserved_2: u8,
    #[allow(dead_code)] reserved_3: u8,
}

struct P00Image {
    data: Vec<u8>,
    offset: u16,
}

impl Image for P00Image {
    fn mount(&mut self, c64: &mut C64) {
        info!(target: "loader", "Mounting P00 image");
        c64.load(&self.data, self.offset);
    }

    #[allow(unused_variables)]
    fn unmount(&mut self, c64: &mut C64) {}
}

pub struct P00Loader {}

impl P00Loader {
    pub fn new() -> P00Loader {
        P00Loader {}
    }

    fn read_header(&self, rdr: &mut Read) -> io::Result<Header> {
        let mut signature = [0u8; 7];
        let mut filename = [0u8; 16];
        let header = Header {
            signature: {
                rdr.read_exact(&mut signature)?;
                signature
            },
            reserved_1: rdr.read_u8()?,
            filename: {
                rdr.read_exact(&mut filename)?;
                filename
            },
            reserved_2: rdr.read_u8()?,
            reserved_3: rdr.read_u8()?,
        };
        Ok(header)
    }

    fn validate_header(&self, header: &Header) -> io::Result<()> {
        let sig = str::from_utf8(&header.signature)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid P00 signature"))?;
        if sig == HEADER_SIG {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "invalid P00 signature",
            ))
        }
    }
}

impl Loader for P00Loader {
    fn autostart(&self, path: &Path) -> Result<AutostartMethod, io::Error> {
        let image = self.load(path)?;
        let autostart = Autostart::new(autostart::Mode::Run, image);
        Ok(AutostartMethod::WithAutostart(Some(autostart)))
    }

    fn load(&self, path: &Path) -> Result<Box<Image>, io::Error> {
        info!(target: "loader", "Loading P00 {}", path.to_str().unwrap());
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let header = self.read_header(&mut reader)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid P00 header"))?;
        self.validate_header(&header)?;
        let offset = reader.read_u16::<LittleEndian>()?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        info!(target: "loader", "Program offset 0x{:x}, size {}", offset, data.len());
        Ok(Box::new(
            P00Image {
                data,
                offset,
            }
        ))
    }
}