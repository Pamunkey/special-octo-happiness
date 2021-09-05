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
    0xde, 0x8f, 0x2c, 0x6a, 0x1d, 0x20, 0xc2, 0x53,