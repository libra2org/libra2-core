// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

fn main() {
    // Test for ripemd160, output_length < 256
    let ripemd = libra2_crypto::hkdf::Hkdf::<ripemd160::Ripemd160>::extract(None, &[]);
    assert!(ripemd.is_ok());
}
