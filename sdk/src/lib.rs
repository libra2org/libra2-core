// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The official Rust SDK for Libra2.
//!
//! ## Modules
//!
//! This SDK provides all the necessary components for building on top of the Libra2 Blockchain. Some of the important modules are:
//!
//! * `crypto` - Types used for signing and verifying
//! * `move_types` - Includes types used when interacting with the Move VM
//! * `rest_client` - The Libra2 API Client, used for sending requests to the Libra2 Blockchain.
//! * `transaction_builder` - Includes helpers for constructing transactions
//! * `types` - Includes types for Libra2 on-chain data structures
//!
//! ## Example
//!
//! Here is a simple example to show how to create two accounts and do a P2p transfer on testnet:
//! todo(davidiw) bring back example using rest
//!

pub use bcs;

pub mod coin_client;

pub mod crypto {
    pub use libra2_crypto::*;
}

pub mod move_types {
    pub use move_core_types::*;
}

pub mod rest_client {
    pub use libra2_rest_client::*;
}

pub mod transaction_builder;

pub mod types;
