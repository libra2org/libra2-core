// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use libra2_faucet_core::server::Server;
use libra2_logger::info;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    server: Server,
}

impl Args {
    pub async fn run_command(&self) -> Result<()> {
        self.server.run_command().await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let root_args = Args::parse();

    libra2_logger::Logger::builder()
        .level(libra2_logger::Level::Info)
        .build();

    info!("Running with root args: {:#?}", root_args);

    root_args.run_command().await
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
