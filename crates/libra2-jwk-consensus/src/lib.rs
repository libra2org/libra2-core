// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    epoch_manager::EpochManager,
    network::{IncomingRpcRequest, NetworkTask},
    network_interface::JWKConsensusNetworkClient,
    types::JWKConsensusMsg,
};
use libra2_channels::libra2_channel;
use libra2_config::config::SafetyRulesConfig;
use libra2_event_notifications::{
    DbBackedOnChainConfig, EventNotificationListener, ReconfigNotificationListener,
};
use libra2_network::application::interface::{NetworkClient, NetworkServiceEvents};
use libra2_types::{
    account_address::AccountAddress,
    jwks::{ObservedJWKs, ObservedJWKsUpdated, SupportedOIDCProviders},
};
use libra2_validator_transaction_pool::VTxnPoolState;
use futures_channel::oneshot;
use tokio::runtime::Runtime;

#[allow(clippy::let_and_return)]
pub fn start_jwk_consensus_runtime(
    my_addr: AccountAddress,
    safety_rules_config: &SafetyRulesConfig,
    network_client: NetworkClient<JWKConsensusMsg>,
    network_service_events: NetworkServiceEvents<JWKConsensusMsg>,
    reconfig_events: ReconfigNotificationListener<DbBackedOnChainConfig>,
    jwk_updated_events: EventNotificationListener,
    vtxn_pool_writer: VTxnPoolState,
) -> Runtime {
    let runtime = libra2_runtimes::spawn_named_runtime("jwk".into(), Some(4));
    let (self_sender, self_receiver) = libra2_channels::new(1_024, &counters::PENDING_SELF_MESSAGES);
    let jwk_consensus_network_client = JWKConsensusNetworkClient::new(network_client);
    let epoch_manager = EpochManager::new(
        my_addr,
        safety_rules_config,
        reconfig_events,
        jwk_updated_events,
        self_sender,
        jwk_consensus_network_client,
        vtxn_pool_writer,
    );
    let (network_task, network_receiver) = NetworkTask::new(network_service_events, self_receiver);
    runtime.spawn(network_task.start());
    runtime.spawn(epoch_manager.start(network_receiver));
    runtime
}

pub mod counters;
pub mod epoch_manager;
pub mod jwk_manager; //TODO: rename to issuer_level_consensus
pub mod jwk_manager_per_key; //TODO: rename to key_level_consensus
pub mod jwk_observer;
pub mod mode;
pub mod network;
pub mod network_interface;
pub mod observation_aggregation;
pub mod types;
pub mod update_certifier;

#[async_trait::async_trait]
trait TConsensusManager: Send + Sync {
    async fn run(
        self: Box<Self>,
        oidc_providers: Option<SupportedOIDCProviders>,
        observed_jwks: Option<ObservedJWKs>,
        mut jwk_updated_rx: libra2_channel::Receiver<(), ObservedJWKsUpdated>,
        mut rpc_req_rx: libra2_channel::Receiver<
            AccountAddress,
            (AccountAddress, IncomingRpcRequest),
        >,
        close_rx: oneshot::Receiver<oneshot::Sender<()>>,
    );
}
