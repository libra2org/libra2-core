// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::ProofRead;
use libra2_crypto::HashValue;
use libra2_types::proof::SparseMerkleProofExt;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProofReader(HashMap<HashValue, SparseMerkleProofExt>);

impl ProofReader {
    pub fn new(key_with_proof: Vec<(HashValue, SparseMerkleProofExt)>) -> Self {
        ProofReader(key_with_proof.into_iter().collect())
    }
}

impl ProofRead for ProofReader {
    fn get_proof(&self, key: &HashValue, root_depth: usize) -> Option<SparseMerkleProofExt> {
        let ret = self.0.get(key);
        if let Some(proof) = ret {
            assert!(proof.root_depth() <= root_depth);
        }
        ret.cloned()
    }
}
