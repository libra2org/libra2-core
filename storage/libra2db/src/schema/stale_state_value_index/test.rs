// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use super::*;
use libra2_schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encode_decode(
        stale_state_value_index in any::<StaleStateValueIndex>(),
    ) {
        assert_encode_decode::<StaleStateValueIndexSchema>(&stale_state_value_index, &());
    }
}

test_no_panic_decoding!(StaleStateValueIndexSchema);
