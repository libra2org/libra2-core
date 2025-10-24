// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

//! Constant values useful for indexing.

use once_cell::sync::Lazy;

/// Type string for Libra2Coin.
pub const LIBRA2_COIN_TYPE_STR: &str = "0x1::libra2_coin::Libra2Coin";

pub static LBT_METADATA_ADDRESS_RAW: Lazy<[u8; 32]> = Lazy::new(|| {
    let mut addr = [0u8; 32];
    addr[31] = 10u8;
    addr
});

pub static LBT_METADATA_ADDRESS_HEX: Lazy<String> =
    Lazy::new(|| format!("0x{}", hex::encode(*LBT_METADATA_ADDRESS_RAW)));
