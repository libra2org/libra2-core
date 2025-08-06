#![forbid(unsafe_code)]

// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_telemetry_service::AptosTelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    libra2_logger::Logger::new().init();
    AptosTelemetryServiceArgs::parse().run().await;
}
