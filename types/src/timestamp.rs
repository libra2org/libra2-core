// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    move_resource::{MoveResource, MoveStructType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimestampResource {
    pub timestamp: Timestamp,
}

impl MoveStructType for TimestampResource {
    const MODULE_NAME: &'static IdentStr = ident_str!("timestamp");
    const STRUCT_NAME: &'static IdentStr = ident_str!("CurrentTimeMicroseconds");
}

impl MoveResource for TimestampResource {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timestamp {
    pub microseconds: u64,
}
