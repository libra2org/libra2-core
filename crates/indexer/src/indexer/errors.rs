// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Error;

// Error, start_version, end_version, name
type ErrorWithVersionAndName = (Error, u64, u64, &'static str);

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionProcessingError {
    /// Could not get a connection
    ConnectionPoolError(ErrorWithVersionAndName),
    /// Could not commit the transaction
    TransactionCommitError(ErrorWithVersionAndName),
}

impl TransactionProcessingError {
    pub fn inner(&self) -> &ErrorWithVersionAndName {
        match self {
            TransactionProcessingError::ConnectionPoolError(ewv) => ewv,
            TransactionProcessingError::TransactionCommitError(ewv) => ewv,
        }
    }
}
