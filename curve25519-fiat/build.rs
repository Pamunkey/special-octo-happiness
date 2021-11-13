// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;
use std::{env, process::Command};

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));

pub const FIAT_REPO: &str = "https://github.com/mit-plv/fiat-crypto.git";
pub const FIAT_HASH: &str = "c96f983228d08c74254004b0bc101d3f6ff8b051";
pub const FIAT_FILE_HASH: [u8; 32] = [
    0xde, 0x8f, 0x2c, 0x6a, 0x1d, 0x20, 0xc2, 0x53, 0x81, 0x85, 0x82, 0xfc, 0x6c, 0x78, 0x91, 0x96,
    0x27, 0xd3, 0xf, 0xc7, 0x5a, 0x27, 0x7b, 0x57, 0x5c, 0xb1, 0x58, 0x5d, 0x6b, 0xd9, 0xa2, 0x2d,
];

use sha2::{Digest, Sha256};
use std::{fs, io};

fn run<F>(name: &str, mut configure: F)
where
    F: FnMut