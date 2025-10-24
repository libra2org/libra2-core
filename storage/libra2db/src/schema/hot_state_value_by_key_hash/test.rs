// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use super::*;
use libra2_crypto::HashValue;
use libra2_schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encode_decode(
        state_key in any::<HashValue>(),
        version in any::<Version>(),
        v in any::<Option<HotStateValue>>(),
    ) {
        assert_encode_decode::<HotStateValueByKeyHashSchema>(&(state_key, version), &v);
    }
}

test_no_panic_decoding!(HotStateValueByKeyHashSchema);
