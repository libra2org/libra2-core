// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::smoke_test_environment::new_local_swarm_with_libra2;
use libra2_cached_packages::libra2_stdlib;
use libra2_forge::{Libra2PublicInfo, Swarm};
use libra2_sdk::{transaction_builder::TransactionBuilder, types::LocalAccount};
use libra2_types::{
    account_address::AccountAddress, account_config::libra2_test_root_address, chain_id::ChainId,
};

async fn submit_and_check_err<F: Fn(TransactionBuilder) -> TransactionBuilder>(
    local_account: &LocalAccount,
    info: &mut Libra2PublicInfo,
    f: F,
    expected: &str,
) {
    let payload = info
        .transaction_factory()
        .payload(libra2_stdlib::libra2_coin_claim_mint_capability())
        .sequence_number(0);
    let txn = local_account.sign_transaction(f(payload).build());
    let err = format!(
        "{:?}",
        info.client().submit_and_wait(&txn).await.unwrap_err()
    );
    assert!(
        err.contains(expected),
        "expected = {}, err = {}",
        expected,
        err
    )
}

#[tokio::test]
async fn test_error_report() {
    let swarm = new_local_swarm_with_libra2(1).await;
    let mut info = swarm.libra2_public_info();

    let local_account = info.random_account();
    let address = local_account.address();
    info.create_user_account(local_account.public_key())
        .await
        .unwrap();
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| t.sender(address),
        "INSUFFICIENT_BALANCE_FOR_TRANSACTION_FEE",
    )
    .await;
    // TODO(Gas): re-enable this
    /*submit_and_check_err(
        &local_account,
        ctx,
        |t| t.sender(address).gas_unit_price(0),
        "GAS_UNIT_PRICE_BELOW_MIN_BOUND",
    )
    .await;*/
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| t.sender(address).chain_id(ChainId::new(100)),
        "BAD_CHAIN_ID",
    )
    .await;
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| t.sender(AccountAddress::random()),
        "INVALID_AUTH_KEY",
    )
    .await;
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| t.sender(libra2_test_root_address()),
        "SEQUENCE_NUMBER_TOO_OLD",
    )
    .await;
    let root_account_sequence_number = info.root_account().sequence_number();
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| {
            t.sender(libra2_test_root_address())
                .sequence_number(root_account_sequence_number)
        },
        "INVALID_AUTH_KEY",
    )
    .await;
    info.mint(address, 100000).await.unwrap();
    submit_and_check_err(
        &local_account,
        &mut info,
        |t| t.sender(address).expiration_timestamp_secs(0),
        "TRANSACTION_EXPIRED",
    )
    .await;
}
