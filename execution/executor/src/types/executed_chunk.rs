// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use crate::types::partial_state_compute_result::PartialStateComputeResult;
use libra2_types::ledger_info::LedgerInfoWithSignatures;

#[derive(Debug)]
pub struct ExecutedChunk {
    pub output: PartialStateComputeResult,
    pub ledger_info_opt: Option<LedgerInfoWithSignatures>,
}
