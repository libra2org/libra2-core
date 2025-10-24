#![forbid(unsafe_code)]

// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_telemetry_service::Libra2TelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    libra2_logger::Logger::new().init();
    Libra2TelemetryServiceArgs::parse().run().await;
}
