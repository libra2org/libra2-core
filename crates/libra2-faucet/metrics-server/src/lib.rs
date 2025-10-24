// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

mod config;
mod gather_metrics;
mod server;

pub use config::MetricsServerConfig;
pub use server::run_metrics_server;
