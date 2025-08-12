// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{assert_success, MoveHarness};
use libra2_cached_packages::libra2_stdlib::libra2_account_transfer;
use libra2_language_e2e_tests::account::Account;

#[test]
fn non_existent_sender() {
    let mut h = MoveHarness::new();

    let sender = Account::new();
    let receiver = h.new_account_with_balance_and_sequence_number(100_000, 0);

    let txn = sender
        .transaction()
        .payload(libra2_account_transfer(*receiver.address(), 0))
        .sequence_number(0)
        .sign();

    let status = h.run(txn);
    assert_success!(status);
}
