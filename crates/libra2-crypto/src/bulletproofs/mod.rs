// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

//! For now, this module stores some constants related to our implementation of Bulletproofs as a
//! Move API.

/// The maximum range we'll use Bulletproofs with is [0, 2^64). Might need to revisit this later.
pub const MAX_RANGE_BITS: usize = 64;
