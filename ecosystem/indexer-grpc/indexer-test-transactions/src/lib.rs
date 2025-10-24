// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

pub mod json_transactions;

#[cfg(test)]
mod tests {
    use super::*;
    use libra2_protos::transaction::v1::Transaction;
    use json_transactions::generated_transactions::IMPORTED_TESTNET_TXNS_5979639459_COIN_REGISTER;
    #[test]
    fn test_generate_transactions() {
        let json_bytes = IMPORTED_TESTNET_TXNS_5979639459_COIN_REGISTER;
        // Check that the transaction is valid JSON
        let transaction = serde_json::from_slice::<Transaction>(json_bytes).unwrap();

        assert_eq!(transaction.version, 5979639459);
    }
}
