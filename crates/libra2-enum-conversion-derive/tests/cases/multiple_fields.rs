// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use libra2_enum_conversion_derive::EnumConversion;

#[derive(EnumConversion)]
enum Messages {
    Test(String, String)
}

fn main() {

}
