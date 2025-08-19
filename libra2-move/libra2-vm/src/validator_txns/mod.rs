// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_vm_ext::{Libra2MoveResolver, SessionId},
    Libra2VM,
};
use libra2_types::validator_txn::ValidatorTransaction;
use libra2_vm_logging::log_schema::AdapterLogSchema;
use libra2_vm_types::{
    module_and_script_storage::module_storage::Libra2ModuleStorage, output::VMOutput,
};
use move_core_types::vm_status::VMStatus;

impl Libra2VM {
    pub(crate) fn process_validator_transaction(
        &self,
        resolver: &impl Libra2MoveResolver,
        module_storage: &impl Libra2ModuleStorage,
        txn: ValidatorTransaction,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, VMOutput), VMStatus> {
        let session_id = SessionId::validator_txn(&txn);
        match txn {
            ValidatorTransaction::DKGResult(dkg_node) => {
                self.process_dkg_result(resolver, module_storage, log_context, session_id, dkg_node)
            },
            ValidatorTransaction::ObservedJWKUpdate(jwk_update) => self.process_jwk_update(
                resolver,
                module_storage,
                log_context,
                session_id,
                jwk_update,
            ),
        }
    }
}

mod dkg;
mod jwk;
