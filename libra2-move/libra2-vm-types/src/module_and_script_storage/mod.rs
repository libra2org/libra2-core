// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

pub mod code_storage;
pub mod module_storage;

mod state_view_adapter;
pub use state_view_adapter::{Libra2CodeStorageAdapter, AsLibra2CodeStorage};
