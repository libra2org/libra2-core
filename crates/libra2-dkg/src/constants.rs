// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use num_bigint::BigUint;
use once_cell::sync::Lazy;

//
// Sizes
//

/// The size in bytes of a compressed G1 point (efficiently deserializable into projective coordinates)
pub const G1_PROJ_NUM_BYTES: usize = 48;

/// The size in bytes of a compressed G2 point (efficiently deserializable into projective coordinates)
pub const G2_PROJ_NUM_BYTES: usize = 96;

/// The size in bytes of a scalar.
pub const SCALAR_NUM_BYTES: usize = 32;

pub(crate) const SCALAR_FIELD_ORDER: Lazy<BigUint> =
    Lazy::new(crate::utils::biguint::get_scalar_field_order_as_biguint);
