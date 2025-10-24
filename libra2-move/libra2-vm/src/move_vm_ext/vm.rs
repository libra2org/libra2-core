// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::move_vm_ext::{Libra2MoveResolver, SessionExt, SessionId};
use libra2_crypto::HashValue;
use libra2_gas_schedule::{MiscGasParameters, NativeGasParameters, LATEST_GAS_FEATURE_VERSION};
use libra2_native_interface::SafeNativeBuilder;
use libra2_types::{
    chain_id::ChainId,
    on_chain_config::{Features, TimedFeaturesBuilder},
    transaction::user_transaction_context::UserTransactionContext,
};
use libra2_vm_environment::{
    environment::Libra2Environment,
    natives::libra2_natives_with_builder,
    prod_configs::{libra2_default_ty_builder, libra2_prod_vm_config},
};
use libra2_vm_types::storage::change_set_configs::ChangeSetConfigs;
use move_vm_runtime::{config::VMConfig, RuntimeEnvironment};

/// Used by genesis to create runtime environment and VM ([GenesisMoveVm]), encapsulating all
/// configs.
pub struct GenesisRuntimeBuilder {
    chain_id: ChainId,
    features: Features,
    runtime_environment: RuntimeEnvironment,
}

impl GenesisRuntimeBuilder {
    /// Returns a builder, capable of creating VM and runtime environment to run genesis.
    pub fn new(chain_id: ChainId) -> Self {
        let features = Features::default();
        let timed_features = TimedFeaturesBuilder::enable_all().build();

        let vm_config = libra2_prod_vm_config(
            LATEST_GAS_FEATURE_VERSION,
            &features,
            &timed_features,
            libra2_default_ty_builder(),
        );

        // All genesis sessions run with unmetered gas meter, and here we set
        // the gas parameters for natives as zeros (because they do not matter).
        let mut native_builder = SafeNativeBuilder::new(
            LATEST_GAS_FEATURE_VERSION,
            NativeGasParameters::zeros(),
            MiscGasParameters::zeros(),
            timed_features.clone(),
            features.clone(),
            None,
        );
        let natives = libra2_natives_with_builder(&mut native_builder, false);
        let runtime_environment = RuntimeEnvironment::new_with_config(natives, vm_config);
        Self {
            chain_id,
            features,
            runtime_environment,
        }
    }

    /// Returns the runtime environment used for any genesis sessions.
    pub fn build_genesis_runtime_environment(&self) -> RuntimeEnvironment {
        self.runtime_environment.clone()
    }

    /// Returns MoveVM for the genesis.
    pub fn build_genesis_vm(&self) -> GenesisMoveVm {
        GenesisMoveVm {
            chain_id: self.chain_id,
            features: self.features.clone(),
            vm_config: self.runtime_environment.vm_config().clone(),
        }
    }
}

/// MoveVM wrapper which is used to run genesis initializations. Designed as a stand-alone struct
/// to ensure all genesis configurations are in one place, and are modified accordingly. The VM is
/// created via [GenesisRuntimeBuilder], and should only be used to run genesis sessions.
pub struct GenesisMoveVm {
    chain_id: ChainId,
    features: Features,
    vm_config: VMConfig,
}

impl GenesisMoveVm {
    /// Returns a new genesis session.
    pub fn new_genesis_session<'r, R: Libra2MoveResolver>(
        &self,
        resolver: &'r R,
        genesis_id: HashValue,
    ) -> SessionExt<'r, R> {
        let session_id = SessionId::genesis(genesis_id);
        SessionExt::new(
            session_id,
            self.chain_id,
            &self.features,
            &self.vm_config,
            None,
            resolver,
        )
    }

    /// Returns the set of features used by genesis VM.
    pub fn genesis_features(&self) -> &Features {
        &self.features
    }

    /// Returns change set configs used by genesis VM sessions. Because genesis sessions are not
    /// metered, there are no change set (storage) costs.
    pub fn genesis_change_set_configs(&self) -> ChangeSetConfigs {
        ChangeSetConfigs::unlimited_at_gas_feature_version(LATEST_GAS_FEATURE_VERSION)
    }
}

pub struct MoveVmExt {
    pub(crate) env: Libra2Environment,
}

impl MoveVmExt {
    pub fn new(env: &Libra2Environment) -> Self {
        Self { env: env.clone() }
    }

    pub fn new_session<'r, R: Libra2MoveResolver>(
        &self,
        resolver: &'r R,
        session_id: SessionId,
        maybe_user_transaction_context: Option<UserTransactionContext>,
    ) -> SessionExt<'r, R> {
        SessionExt::new(
            session_id,
            self.env.chain_id(),
            self.env.features(),
            self.env.vm_config(),
            maybe_user_transaction_context,
            resolver,
        )
    }
}
