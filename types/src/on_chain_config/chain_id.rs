// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{chain_id::ChainId, on_chain_config::OnChainConfig};

impl OnChainConfig for ChainId {
    const MODULE_IDENTIFIER: &'static str = "chain_id";
    const TYPE_IDENTIFIER: &'static str = "ChainId";
}
