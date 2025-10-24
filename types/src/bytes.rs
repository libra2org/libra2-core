// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use bytes::Bytes;

pub trait NumToBytes {
    fn le_bytes(&self) -> Bytes;
}

impl NumToBytes for u64 {
    fn le_bytes(&self) -> Bytes {
        Bytes::copy_from_slice(&self.to_le_bytes())
    }
}
