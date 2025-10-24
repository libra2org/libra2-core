// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

mod get_leaf;
mod get_path;
mod get_snapshots;

use libra2_storage_interface::Result;

#[derive(clap::Subcommand)]
pub enum Cmd {
    GetSnapshots(get_snapshots::Cmd),
    GetPath(get_path::Cmd),
    GetLeaf(get_leaf::Cmd),
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        match self {
            Self::GetSnapshots(cmd) => Ok(cmd.run()?),
            Self::GetPath(cmd) => cmd.run(),
            Self::GetLeaf(cmd) => cmd.run(),
        }
    }
}
