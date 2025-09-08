// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{components::get_signer_arg, utils::*};
use anyhow::Result;
use libra2_crypto::HashValue;
use libra2_types::on_chain_config::{FeatureFlag as Libra2FeatureFlag, Features as Libra2Features};
use move_model::{code_writer::CodeWriter, emit, emitln, model::Loc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, Debug)]
pub struct Features {
    #[serde(default)]
    pub enabled: Vec<FeatureFlag>,
    #[serde(default)]
    pub disabled: Vec<FeatureFlag>,
}

impl Features {
    pub fn empty() -> Self {
        Self {
            enabled: vec![],
            disabled: vec![],
        }
    }

    pub fn squash(&mut self, rhs: Self) {
        let mut enabled: HashSet<_> = self.enabled.iter().cloned().collect();
        let mut disabled: HashSet<_> = self.disabled.iter().cloned().collect();
        let to_enable: HashSet<_> = rhs.enabled.into_iter().collect();
        let to_disable: HashSet<_> = rhs.disabled.into_iter().collect();

        disabled = disabled.difference(&to_enable).cloned().collect();
        enabled.extend(to_enable);

        enabled = enabled.difference(&to_disable).cloned().collect();
        disabled.extend(to_disable);

        self.enabled = enabled.into_iter().collect();
        self.disabled = disabled.into_iter().collect();
    }

    pub fn is_empty(&self) -> bool {
        self.enabled.is_empty() && self.disabled.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, EnumIter, PartialEq, Eq, Serialize, Hash)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    CodeDependencyCheck,
    CollectAndDistributeGasFees,
    TreatFriendAsPrivate,
    Sha512AndRipeMd160Natives,
    Libra2StdChainIdNatives,
    VMBinaryFormatV6,
    MultiEd25519PkValidateV2Natives,
    Blake2b256Native,
    ResourceGroups,
    MultisigAccounts,
    DelegationPools,
    CryptographyAlgebraNatives,
    Bls12381Structures,
    Ed25519PubkeyValidateReturnFalseWrongLength,
    StructConstructors,
    PeriodicalRewardRateReduction,
    PartialGovernanceVoting,
    SignatureCheckerV2,
    StorageSlotMetadata,
    ChargeInvariantViolation,
    DelegationPoolPartialGovernanceVoting,
    GasPayerEnabled,
    Libra2UniqueIdentifiers,
    BulletproofsNatives,
    SignerNativeFormatFix,
    ModuleEvent,
    EmitFeeStatement,
    StorageDeletionRefund,
    AggregatorV2Api,
    SignatureCheckerV2ScriptFix,
    SaferResourceGroups,
    SaferMetadata,
    SingleSenderAuthenticator,
    SponsoredAutomaticAccountCreation,
    FeePayerAccountOptional,
    AggregatorV2DelayedFields,
    ConcurrentTokenV2,
    LimitMaxIdentifierLength,
    OperatorBeneficiaryChange,
    VMBinaryFormatV7,
    ResourceGroupsSplitInVmChangeSet,
    CommissionChangeDelegationPool,
    Bn254Structures,
    WebAuthnSignature,
    ReconfigureWithDkg,
    KeylessAccounts,
    KeylessButZklessAccounts,
    RemoveDetailedError,
    JwkConsensus,
    ConcurrentFungibleAssets,
    RefundableBytes,
    ObjectCodeDeployment,
    MaxObjectNestingCheck,
    KeylessAccountsWithPasskeys,
    MultisigV2Enhancement,
    DelegationPoolAllowlisting,
    ModuleEventMigration,
    RejectUnstableBytecode,
    TransactionContextExtension,
    CoinToFungibleAssetMigration,
    PrimaryAPTFungibleStoreAtUserAddress,
    ObjectNativeDerivedAddress,
    DispatchableFungibleAsset,
    NewAccountsDefaultToFaAptStore,
    OperationsDefaultToFaAptStore,
    AggregatorV2IsAtLeastApi,
    ConcurrentFungibleBalance,
    DefaultToConcurrentFungibleBalance,
    LimitVMTypeSize,
    AbortIfMultisigPayloadMismatch,
    DisallowUserNative,
    AllowSerializedScriptArgs,
    UseCompatibilityCheckerV2,
    EnableEnumTypes,
    EnableResourceAccessControl,
    RejectUnstableBytecodeForScript,
    FederatedKeyless,
    TransactionSimulationEnhancement,
    CollectionOwner,
    NativeMemoryOperations,
    EnableLoaderV2,
    DisallowInitModuleToPublishModules,
    EnableCallTreeAndInstructionVMCache,
    PermissionedSigner,
    AccountAbstraction,
    VMBinaryFormatV8,
    BulletproofsBatchNatives,
    DerivableAccountAbstraction,
    EnableFunctionValues,
    NewAccountsDefaultToFaStore,
    DefaultAccountResource,
    JwkConsensusPerKeyMode,
    TransactionPayloadV2,
    OrderlessTransactions,
    EnableLazyLoading,
    CalculateTransactionFeeForDistribution,
    DistributeTransactionFee,
}

fn generate_features_blob(writer: &CodeWriter, data: &[u64]) {
    emitln!(writer, "vector[");
    writer.indent();
    for (i, b) in data.iter().enumerate() {
        if i % 20 == 0 {
            if i > 0 {
                emitln!(writer);
            }
        } else {
            emit!(writer, " ");
        }
        emit!(writer, "{},", b);
    }
    emitln!(writer);
    writer.unindent();
    emit!(writer, "]")
}

pub fn generate_feature_upgrade_proposal(
    features: &Features,
    is_testnet: bool,
    next_execution_hash: Option<HashValue>,
    is_multi_step: bool,
) -> Result<Vec<(String, String)>> {
    let signer_arg = get_signer_arg(is_testnet, &next_execution_hash);
    let mut result = vec![];

    let enabled = features
        .enabled
        .iter()
        .map(|f| Libra2FeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();
    let disabled = features
        .disabled
        .iter()
        .map(|f| Libra2FeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();

    assert!(enabled.len() < u16::MAX as usize);
    assert!(disabled.len() < u16::MAX as usize);

    let writer = CodeWriter::new(Loc::default());

    emitln!(writer, "// Modifying on-chain feature flags: ");
    emitln!(writer, "// Enabled Features: {:?}", features.enabled);
    emitln!(writer, "// Disabled Features: {:?}", features.disabled);
    emitln!(writer, "//");

    let proposal = generate_governance_proposal(
        &writer,
        is_testnet,
        next_execution_hash,
        is_multi_step,
        &["std::features"],
        |writer| {
            emit!(writer, "let enabled_blob: vector<u64> = ");
            generate_features_blob(writer, &enabled);
            emitln!(writer, ";\n");

            emit!(writer, "let disabled_blob: vector<u64> = ");
            generate_features_blob(writer, &disabled);
            emitln!(writer, ";\n");

            emitln!(
                writer,
                "features::change_feature_flags_for_next_epoch({}, enabled_blob, disabled_blob);",
                signer_arg
            );
            emitln!(writer, "libra2_governance::reconfigure({});", signer_arg);
        },
    );

    result.push(("features".to_string(), proposal));
    Ok(result)
}

impl From<FeatureFlag> for Libra2FeatureFlag {
    fn from(f: FeatureFlag) -> Self {
        match f {
            FeatureFlag::CodeDependencyCheck => Libra2FeatureFlag::CODE_DEPENDENCY_CHECK,
            FeatureFlag::CollectAndDistributeGasFees => {
                Libra2FeatureFlag::_DEPRECATED_COLLECT_AND_DISTRIBUTE_GAS_FEES
            },
            FeatureFlag::TreatFriendAsPrivate => Libra2FeatureFlag::TREAT_FRIEND_AS_PRIVATE,
            FeatureFlag::Sha512AndRipeMd160Natives => {
                Libra2FeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES
            },
            FeatureFlag::Libra2StdChainIdNatives => Libra2FeatureFlag::LIBRA2_STD_CHAIN_ID_NATIVES,
            FeatureFlag::VMBinaryFormatV6 => Libra2FeatureFlag::VM_BINARY_FORMAT_V6,
            FeatureFlag::VMBinaryFormatV7 => Libra2FeatureFlag::VM_BINARY_FORMAT_V7,
            FeatureFlag::VMBinaryFormatV8 => Libra2FeatureFlag::VM_BINARY_FORMAT_V8,
            FeatureFlag::MultiEd25519PkValidateV2Natives => {
                Libra2FeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES
            },
            FeatureFlag::Blake2b256Native => Libra2FeatureFlag::BLAKE2B_256_NATIVE,
            FeatureFlag::ResourceGroups => Libra2FeatureFlag::RESOURCE_GROUPS,
            FeatureFlag::MultisigAccounts => Libra2FeatureFlag::MULTISIG_ACCOUNTS,
            FeatureFlag::DelegationPools => Libra2FeatureFlag::DELEGATION_POOLS,
            FeatureFlag::CryptographyAlgebraNatives => {
                Libra2FeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES
            },
            FeatureFlag::Bls12381Structures => Libra2FeatureFlag::BLS12_381_STRUCTURES,
            FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength => {
                Libra2FeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH
            },
            FeatureFlag::StructConstructors => Libra2FeatureFlag::STRUCT_CONSTRUCTORS,
            FeatureFlag::PeriodicalRewardRateReduction => {
                Libra2FeatureFlag::PERIODICAL_REWARD_RATE_DECREASE
            },
            FeatureFlag::PartialGovernanceVoting => Libra2FeatureFlag::PARTIAL_GOVERNANCE_VOTING,
            FeatureFlag::SignatureCheckerV2 => Libra2FeatureFlag::SIGNATURE_CHECKER_V2,
            FeatureFlag::StorageSlotMetadata => Libra2FeatureFlag::STORAGE_SLOT_METADATA,
            FeatureFlag::ChargeInvariantViolation => Libra2FeatureFlag::CHARGE_INVARIANT_VIOLATION,
            FeatureFlag::DelegationPoolPartialGovernanceVoting => {
                Libra2FeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING
            },
            FeatureFlag::GasPayerEnabled => Libra2FeatureFlag::GAS_PAYER_ENABLED,
            FeatureFlag::Libra2UniqueIdentifiers => Libra2FeatureFlag::LIBRA2_UNIQUE_IDENTIFIERS,
            FeatureFlag::BulletproofsNatives => Libra2FeatureFlag::BULLETPROOFS_NATIVES,
            FeatureFlag::SignerNativeFormatFix => Libra2FeatureFlag::SIGNER_NATIVE_FORMAT_FIX,
            FeatureFlag::ModuleEvent => Libra2FeatureFlag::MODULE_EVENT,
            FeatureFlag::EmitFeeStatement => Libra2FeatureFlag::EMIT_FEE_STATEMENT,
            FeatureFlag::StorageDeletionRefund => Libra2FeatureFlag::STORAGE_DELETION_REFUND,
            FeatureFlag::AggregatorV2Api => Libra2FeatureFlag::AGGREGATOR_V2_API,
            FeatureFlag::SignatureCheckerV2ScriptFix => {
                Libra2FeatureFlag::SIGNATURE_CHECKER_V2_SCRIPT_FIX
            },
            FeatureFlag::SaferResourceGroups => Libra2FeatureFlag::SAFER_RESOURCE_GROUPS,
            FeatureFlag::SaferMetadata => Libra2FeatureFlag::SAFER_METADATA,
            FeatureFlag::SingleSenderAuthenticator => Libra2FeatureFlag::SINGLE_SENDER_AUTHENTICATOR,
            FeatureFlag::SponsoredAutomaticAccountCreation => {
                Libra2FeatureFlag::SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION
            },
            FeatureFlag::FeePayerAccountOptional => Libra2FeatureFlag::FEE_PAYER_ACCOUNT_OPTIONAL,
            FeatureFlag::AggregatorV2DelayedFields => {
                Libra2FeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS
            },
            FeatureFlag::ConcurrentTokenV2 => Libra2FeatureFlag::CONCURRENT_TOKEN_V2,
            FeatureFlag::LimitMaxIdentifierLength => Libra2FeatureFlag::LIMIT_MAX_IDENTIFIER_LENGTH,
            FeatureFlag::OperatorBeneficiaryChange => Libra2FeatureFlag::OPERATOR_BENEFICIARY_CHANGE,
            FeatureFlag::ResourceGroupsSplitInVmChangeSet => {
                Libra2FeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET
            },
            FeatureFlag::CommissionChangeDelegationPool => {
                Libra2FeatureFlag::COMMISSION_CHANGE_DELEGATION_POOL
            },
            FeatureFlag::Bn254Structures => Libra2FeatureFlag::BN254_STRUCTURES,
            FeatureFlag::WebAuthnSignature => Libra2FeatureFlag::WEBAUTHN_SIGNATURE,
            FeatureFlag::ReconfigureWithDkg => Libra2FeatureFlag::_DEPRECATED_RECONFIGURE_WITH_DKG,
            FeatureFlag::KeylessAccounts => Libra2FeatureFlag::KEYLESS_ACCOUNTS,
            FeatureFlag::KeylessButZklessAccounts => Libra2FeatureFlag::KEYLESS_BUT_ZKLESS_ACCOUNTS,
            FeatureFlag::RemoveDetailedError => {
                Libra2FeatureFlag::_DEPRECATED_REMOVE_DETAILED_ERROR_FROM_HASH
            },
            FeatureFlag::JwkConsensus => Libra2FeatureFlag::JWK_CONSENSUS,
            FeatureFlag::ConcurrentFungibleAssets => Libra2FeatureFlag::CONCURRENT_FUNGIBLE_ASSETS,
            FeatureFlag::RefundableBytes => Libra2FeatureFlag::REFUNDABLE_BYTES,
            FeatureFlag::ObjectCodeDeployment => Libra2FeatureFlag::OBJECT_CODE_DEPLOYMENT,
            FeatureFlag::MaxObjectNestingCheck => Libra2FeatureFlag::MAX_OBJECT_NESTING_CHECK,
            FeatureFlag::KeylessAccountsWithPasskeys => {
                Libra2FeatureFlag::KEYLESS_ACCOUNTS_WITH_PASSKEYS
            },
            FeatureFlag::MultisigV2Enhancement => Libra2FeatureFlag::MULTISIG_V2_ENHANCEMENT,
            FeatureFlag::DelegationPoolAllowlisting => {
                Libra2FeatureFlag::DELEGATION_POOL_ALLOWLISTING
            },
            FeatureFlag::ModuleEventMigration => Libra2FeatureFlag::MODULE_EVENT_MIGRATION,
            FeatureFlag::RejectUnstableBytecode => Libra2FeatureFlag::_REJECT_UNSTABLE_BYTECODE,
            FeatureFlag::TransactionContextExtension => {
                Libra2FeatureFlag::TRANSACTION_CONTEXT_EXTENSION
            },
            FeatureFlag::CoinToFungibleAssetMigration => {
                Libra2FeatureFlag::COIN_TO_FUNGIBLE_ASSET_MIGRATION
            },
            FeatureFlag::PrimaryAPTFungibleStoreAtUserAddress => {
                Libra2FeatureFlag::PRIMARY_LBT_FUNGIBLE_STORE_AT_USER_ADDRESS
            },
            FeatureFlag::ObjectNativeDerivedAddress => {
                Libra2FeatureFlag::OBJECT_NATIVE_DERIVED_ADDRESS
            },
            FeatureFlag::DispatchableFungibleAsset => Libra2FeatureFlag::DISPATCHABLE_FUNGIBLE_ASSET,
            FeatureFlag::NewAccountsDefaultToFaAptStore => {
                Libra2FeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_LBT_STORE
            },
            FeatureFlag::OperationsDefaultToFaAptStore => {
                Libra2FeatureFlag::OPERATIONS_DEFAULT_TO_FA_LBT_STORE
            },
            FeatureFlag::AggregatorV2IsAtLeastApi => {
                Libra2FeatureFlag::AGGREGATOR_V2_IS_AT_LEAST_API
            },
            FeatureFlag::ConcurrentFungibleBalance => Libra2FeatureFlag::CONCURRENT_FUNGIBLE_BALANCE,
            FeatureFlag::DefaultToConcurrentFungibleBalance => {
                Libra2FeatureFlag::DEFAULT_TO_CONCURRENT_FUNGIBLE_BALANCE
            },
            FeatureFlag::LimitVMTypeSize => Libra2FeatureFlag::_LIMIT_VM_TYPE_SIZE,
            FeatureFlag::AbortIfMultisigPayloadMismatch => {
                Libra2FeatureFlag::ABORT_IF_MULTISIG_PAYLOAD_MISMATCH
            },
            FeatureFlag::DisallowUserNative => Libra2FeatureFlag::_DISALLOW_USER_NATIVES,
            FeatureFlag::AllowSerializedScriptArgs => {
                Libra2FeatureFlag::ALLOW_SERIALIZED_SCRIPT_ARGS
            },
            FeatureFlag::UseCompatibilityCheckerV2 => {
                Libra2FeatureFlag::_USE_COMPATIBILITY_CHECKER_V2
            },
            FeatureFlag::EnableEnumTypes => Libra2FeatureFlag::ENABLE_ENUM_TYPES,
            FeatureFlag::EnableResourceAccessControl => {
                Libra2FeatureFlag::ENABLE_RESOURCE_ACCESS_CONTROL
            },
            FeatureFlag::RejectUnstableBytecodeForScript => {
                Libra2FeatureFlag::_REJECT_UNSTABLE_BYTECODE_FOR_SCRIPT
            },
            FeatureFlag::FederatedKeyless => Libra2FeatureFlag::FEDERATED_KEYLESS,
            FeatureFlag::TransactionSimulationEnhancement => {
                Libra2FeatureFlag::TRANSACTION_SIMULATION_ENHANCEMENT
            },
            FeatureFlag::CollectionOwner => Libra2FeatureFlag::COLLECTION_OWNER,
            FeatureFlag::NativeMemoryOperations => Libra2FeatureFlag::NATIVE_MEMORY_OPERATIONS,
            FeatureFlag::EnableLoaderV2 => Libra2FeatureFlag::_ENABLE_LOADER_V2,
            FeatureFlag::DisallowInitModuleToPublishModules => {
                Libra2FeatureFlag::_DISALLOW_INIT_MODULE_TO_PUBLISH_MODULES
            },
            FeatureFlag::EnableCallTreeAndInstructionVMCache => {
                Libra2FeatureFlag::ENABLE_CALL_TREE_AND_INSTRUCTION_VM_CACHE
            },
            FeatureFlag::PermissionedSigner => Libra2FeatureFlag::PERMISSIONED_SIGNER,
            FeatureFlag::AccountAbstraction => Libra2FeatureFlag::ACCOUNT_ABSTRACTION,
            FeatureFlag::BulletproofsBatchNatives => Libra2FeatureFlag::BULLETPROOFS_BATCH_NATIVES,
            FeatureFlag::DerivableAccountAbstraction => {
                Libra2FeatureFlag::DERIVABLE_ACCOUNT_ABSTRACTION
            },
            FeatureFlag::EnableFunctionValues => Libra2FeatureFlag::ENABLE_FUNCTION_VALUES,
            FeatureFlag::NewAccountsDefaultToFaStore => {
                Libra2FeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_STORE
            },
            FeatureFlag::DefaultAccountResource => Libra2FeatureFlag::DEFAULT_ACCOUNT_RESOURCE,
            FeatureFlag::JwkConsensusPerKeyMode => Libra2FeatureFlag::JWK_CONSENSUS_PER_KEY_MODE,
            FeatureFlag::TransactionPayloadV2 => Libra2FeatureFlag::TRANSACTION_PAYLOAD_V2,
            FeatureFlag::OrderlessTransactions => Libra2FeatureFlag::ORDERLESS_TRANSACTIONS,
            FeatureFlag::EnableLazyLoading => Libra2FeatureFlag::ENABLE_LAZY_LOADING,
            FeatureFlag::CalculateTransactionFeeForDistribution => {
                Libra2FeatureFlag::CALCULATE_TRANSACTION_FEE_FOR_DISTRIBUTION
            },
            FeatureFlag::DistributeTransactionFee => Libra2FeatureFlag::DISTRIBUTE_TRANSACTION_FEE,
        }
    }
}

// We don't need this implementation. Just to make sure we have an exhaustive 1-1 mapping between the two structs.
impl From<Libra2FeatureFlag> for FeatureFlag {
    fn from(f: Libra2FeatureFlag) -> Self {
        match f {
            Libra2FeatureFlag::CODE_DEPENDENCY_CHECK => FeatureFlag::CodeDependencyCheck,
            Libra2FeatureFlag::_DEPRECATED_COLLECT_AND_DISTRIBUTE_GAS_FEES => {
                FeatureFlag::CollectAndDistributeGasFees
            },
            Libra2FeatureFlag::TREAT_FRIEND_AS_PRIVATE => FeatureFlag::TreatFriendAsPrivate,
            Libra2FeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES => {
                FeatureFlag::Sha512AndRipeMd160Natives
            },
            Libra2FeatureFlag::LIBRA2_STD_CHAIN_ID_NATIVES => FeatureFlag::Libra2StdChainIdNatives,
            Libra2FeatureFlag::VM_BINARY_FORMAT_V6 => FeatureFlag::VMBinaryFormatV6,
            Libra2FeatureFlag::VM_BINARY_FORMAT_V7 => FeatureFlag::VMBinaryFormatV7,
            Libra2FeatureFlag::VM_BINARY_FORMAT_V8 => FeatureFlag::VMBinaryFormatV8,
            Libra2FeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES => {
                FeatureFlag::MultiEd25519PkValidateV2Natives
            },
            Libra2FeatureFlag::BLAKE2B_256_NATIVE => FeatureFlag::Blake2b256Native,
            Libra2FeatureFlag::RESOURCE_GROUPS => FeatureFlag::ResourceGroups,
            Libra2FeatureFlag::MULTISIG_ACCOUNTS => FeatureFlag::MultisigAccounts,
            Libra2FeatureFlag::DELEGATION_POOLS => FeatureFlag::DelegationPools,
            Libra2FeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES => {
                FeatureFlag::CryptographyAlgebraNatives
            },
            Libra2FeatureFlag::BLS12_381_STRUCTURES => FeatureFlag::Bls12381Structures,
            Libra2FeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH => {
                FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength
            },
            Libra2FeatureFlag::STRUCT_CONSTRUCTORS => FeatureFlag::StructConstructors,
            Libra2FeatureFlag::PERIODICAL_REWARD_RATE_DECREASE => {
                FeatureFlag::PeriodicalRewardRateReduction
            },
            Libra2FeatureFlag::PARTIAL_GOVERNANCE_VOTING => FeatureFlag::PartialGovernanceVoting,
            Libra2FeatureFlag::SIGNATURE_CHECKER_V2 => FeatureFlag::SignatureCheckerV2,
            Libra2FeatureFlag::STORAGE_SLOT_METADATA => FeatureFlag::StorageSlotMetadata,
            Libra2FeatureFlag::CHARGE_INVARIANT_VIOLATION => FeatureFlag::ChargeInvariantViolation,
            Libra2FeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING => {
                FeatureFlag::DelegationPoolPartialGovernanceVoting
            },
            Libra2FeatureFlag::GAS_PAYER_ENABLED => FeatureFlag::GasPayerEnabled,
            Libra2FeatureFlag::LIBRA2_UNIQUE_IDENTIFIERS => FeatureFlag::Libra2UniqueIdentifiers,
            Libra2FeatureFlag::BULLETPROOFS_NATIVES => FeatureFlag::BulletproofsNatives,
            Libra2FeatureFlag::SIGNER_NATIVE_FORMAT_FIX => FeatureFlag::SignerNativeFormatFix,
            Libra2FeatureFlag::MODULE_EVENT => FeatureFlag::ModuleEvent,
            Libra2FeatureFlag::EMIT_FEE_STATEMENT => FeatureFlag::EmitFeeStatement,
            Libra2FeatureFlag::STORAGE_DELETION_REFUND => FeatureFlag::StorageDeletionRefund,
            Libra2FeatureFlag::AGGREGATOR_V2_API => FeatureFlag::AggregatorV2Api,
            Libra2FeatureFlag::SIGNATURE_CHECKER_V2_SCRIPT_FIX => {
                FeatureFlag::SignatureCheckerV2ScriptFix
            },
            Libra2FeatureFlag::SAFER_RESOURCE_GROUPS => FeatureFlag::SaferResourceGroups,
            Libra2FeatureFlag::SAFER_METADATA => FeatureFlag::SaferMetadata,
            Libra2FeatureFlag::SINGLE_SENDER_AUTHENTICATOR => FeatureFlag::SingleSenderAuthenticator,
            Libra2FeatureFlag::SPONSORED_AUTOMATIC_ACCOUNT_V1_CREATION => {
                FeatureFlag::SponsoredAutomaticAccountCreation
            },
            Libra2FeatureFlag::FEE_PAYER_ACCOUNT_OPTIONAL => FeatureFlag::FeePayerAccountOptional,
            Libra2FeatureFlag::AGGREGATOR_V2_DELAYED_FIELDS => {
                FeatureFlag::AggregatorV2DelayedFields
            },
            Libra2FeatureFlag::CONCURRENT_TOKEN_V2 => FeatureFlag::ConcurrentTokenV2,
            Libra2FeatureFlag::LIMIT_MAX_IDENTIFIER_LENGTH => FeatureFlag::LimitMaxIdentifierLength,
            Libra2FeatureFlag::OPERATOR_BENEFICIARY_CHANGE => FeatureFlag::OperatorBeneficiaryChange,
            Libra2FeatureFlag::RESOURCE_GROUPS_SPLIT_IN_VM_CHANGE_SET => {
                FeatureFlag::ResourceGroupsSplitInVmChangeSet
            },
            Libra2FeatureFlag::COMMISSION_CHANGE_DELEGATION_POOL => {
                FeatureFlag::CommissionChangeDelegationPool
            },
            Libra2FeatureFlag::BN254_STRUCTURES => FeatureFlag::Bn254Structures,
            Libra2FeatureFlag::WEBAUTHN_SIGNATURE => FeatureFlag::WebAuthnSignature,
            Libra2FeatureFlag::_DEPRECATED_RECONFIGURE_WITH_DKG => FeatureFlag::ReconfigureWithDkg,
            Libra2FeatureFlag::KEYLESS_ACCOUNTS => FeatureFlag::KeylessAccounts,
            Libra2FeatureFlag::KEYLESS_BUT_ZKLESS_ACCOUNTS => FeatureFlag::KeylessButZklessAccounts,
            Libra2FeatureFlag::_DEPRECATED_REMOVE_DETAILED_ERROR_FROM_HASH => {
                FeatureFlag::RemoveDetailedError
            },
            Libra2FeatureFlag::JWK_CONSENSUS => FeatureFlag::JwkConsensus,
            Libra2FeatureFlag::CONCURRENT_FUNGIBLE_ASSETS => FeatureFlag::ConcurrentFungibleAssets,
            Libra2FeatureFlag::REFUNDABLE_BYTES => FeatureFlag::RefundableBytes,
            Libra2FeatureFlag::OBJECT_CODE_DEPLOYMENT => FeatureFlag::ObjectCodeDeployment,
            Libra2FeatureFlag::MAX_OBJECT_NESTING_CHECK => FeatureFlag::MaxObjectNestingCheck,
            Libra2FeatureFlag::KEYLESS_ACCOUNTS_WITH_PASSKEYS => {
                FeatureFlag::KeylessAccountsWithPasskeys
            },
            Libra2FeatureFlag::MULTISIG_V2_ENHANCEMENT => FeatureFlag::MultisigV2Enhancement,
            Libra2FeatureFlag::DELEGATION_POOL_ALLOWLISTING => {
                FeatureFlag::DelegationPoolAllowlisting
            },
            Libra2FeatureFlag::MODULE_EVENT_MIGRATION => FeatureFlag::ModuleEventMigration,
            Libra2FeatureFlag::_REJECT_UNSTABLE_BYTECODE => FeatureFlag::RejectUnstableBytecode,
            Libra2FeatureFlag::TRANSACTION_CONTEXT_EXTENSION => {
                FeatureFlag::TransactionContextExtension
            },
            Libra2FeatureFlag::COIN_TO_FUNGIBLE_ASSET_MIGRATION => {
                FeatureFlag::CoinToFungibleAssetMigration
            },
            Libra2FeatureFlag::PRIMARY_LBT_FUNGIBLE_STORE_AT_USER_ADDRESS => {
                FeatureFlag::PrimaryAPTFungibleStoreAtUserAddress
            },
            Libra2FeatureFlag::OBJECT_NATIVE_DERIVED_ADDRESS => {
                FeatureFlag::ObjectNativeDerivedAddress
            },
            Libra2FeatureFlag::DISPATCHABLE_FUNGIBLE_ASSET => FeatureFlag::DispatchableFungibleAsset,
            Libra2FeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_LBT_STORE => {
                FeatureFlag::NewAccountsDefaultToFaAptStore
            },
            Libra2FeatureFlag::OPERATIONS_DEFAULT_TO_FA_LBT_STORE => {
                FeatureFlag::OperationsDefaultToFaAptStore
            },
            Libra2FeatureFlag::AGGREGATOR_V2_IS_AT_LEAST_API => {
                FeatureFlag::AggregatorV2IsAtLeastApi
            },
            Libra2FeatureFlag::CONCURRENT_FUNGIBLE_BALANCE => FeatureFlag::ConcurrentFungibleBalance,
            Libra2FeatureFlag::DEFAULT_TO_CONCURRENT_FUNGIBLE_BALANCE => {
                FeatureFlag::DefaultToConcurrentFungibleBalance
            },
            Libra2FeatureFlag::_LIMIT_VM_TYPE_SIZE => FeatureFlag::LimitVMTypeSize,
            Libra2FeatureFlag::ABORT_IF_MULTISIG_PAYLOAD_MISMATCH => {
                FeatureFlag::AbortIfMultisigPayloadMismatch
            },
            Libra2FeatureFlag::_DISALLOW_USER_NATIVES => FeatureFlag::DisallowUserNative,
            Libra2FeatureFlag::ALLOW_SERIALIZED_SCRIPT_ARGS => {
                FeatureFlag::AllowSerializedScriptArgs
            },
            Libra2FeatureFlag::_USE_COMPATIBILITY_CHECKER_V2 => {
                FeatureFlag::UseCompatibilityCheckerV2
            },
            Libra2FeatureFlag::ENABLE_ENUM_TYPES => FeatureFlag::EnableEnumTypes,
            Libra2FeatureFlag::ENABLE_RESOURCE_ACCESS_CONTROL => {
                FeatureFlag::EnableResourceAccessControl
            },
            Libra2FeatureFlag::_REJECT_UNSTABLE_BYTECODE_FOR_SCRIPT => {
                FeatureFlag::RejectUnstableBytecodeForScript
            },
            Libra2FeatureFlag::FEDERATED_KEYLESS => FeatureFlag::FederatedKeyless,
            Libra2FeatureFlag::TRANSACTION_SIMULATION_ENHANCEMENT => {
                FeatureFlag::TransactionSimulationEnhancement
            },
            Libra2FeatureFlag::COLLECTION_OWNER => FeatureFlag::CollectionOwner,
            Libra2FeatureFlag::NATIVE_MEMORY_OPERATIONS => FeatureFlag::NativeMemoryOperations,
            Libra2FeatureFlag::_ENABLE_LOADER_V2 => FeatureFlag::EnableLoaderV2,
            Libra2FeatureFlag::_DISALLOW_INIT_MODULE_TO_PUBLISH_MODULES => {
                FeatureFlag::DisallowInitModuleToPublishModules
            },
            Libra2FeatureFlag::ENABLE_CALL_TREE_AND_INSTRUCTION_VM_CACHE => {
                FeatureFlag::EnableCallTreeAndInstructionVMCache
            },
            Libra2FeatureFlag::PERMISSIONED_SIGNER => FeatureFlag::PermissionedSigner,
            Libra2FeatureFlag::ACCOUNT_ABSTRACTION => FeatureFlag::AccountAbstraction,
            Libra2FeatureFlag::BULLETPROOFS_BATCH_NATIVES => FeatureFlag::BulletproofsBatchNatives,
            Libra2FeatureFlag::DERIVABLE_ACCOUNT_ABSTRACTION => {
                FeatureFlag::DerivableAccountAbstraction
            },
            Libra2FeatureFlag::ENABLE_FUNCTION_VALUES => FeatureFlag::EnableFunctionValues,
            Libra2FeatureFlag::NEW_ACCOUNTS_DEFAULT_TO_FA_STORE => {
                FeatureFlag::NewAccountsDefaultToFaStore
            },
            Libra2FeatureFlag::DEFAULT_ACCOUNT_RESOURCE => FeatureFlag::DefaultAccountResource,
            Libra2FeatureFlag::JWK_CONSENSUS_PER_KEY_MODE => FeatureFlag::JwkConsensusPerKeyMode,
            Libra2FeatureFlag::TRANSACTION_PAYLOAD_V2 => FeatureFlag::TransactionPayloadV2,
            Libra2FeatureFlag::ORDERLESS_TRANSACTIONS => FeatureFlag::OrderlessTransactions,
            Libra2FeatureFlag::ENABLE_LAZY_LOADING => FeatureFlag::EnableLazyLoading,
            Libra2FeatureFlag::CALCULATE_TRANSACTION_FEE_FOR_DISTRIBUTION => {
                FeatureFlag::CalculateTransactionFeeForDistribution
            },
            Libra2FeatureFlag::DISTRIBUTE_TRANSACTION_FEE => FeatureFlag::DistributeTransactionFee,
        }
    }
}

impl Features {
    // Compare if the current feature set is different from features that has been enabled on chain.
    pub(crate) fn has_modified(&self, on_chain_features: &Libra2Features) -> bool {
        self.enabled
            .iter()
            .any(|f| !on_chain_features.is_enabled(Libra2FeatureFlag::from(f.clone())))
            || self
                .disabled
                .iter()
                .any(|f| on_chain_features.is_enabled(Libra2FeatureFlag::from(f.clone())))
    }
}

impl From<&Libra2Features> for Features {
    fn from(features: &Libra2Features) -> Features {
        let mut enabled = vec![];
        let mut disabled = vec![];
        for feature in FeatureFlag::iter() {
            if features.is_enabled(Libra2FeatureFlag::from(feature.clone())) {
                enabled.push(feature);
            } else {
                disabled.push(feature);
            }
        }
        Features { enabled, disabled }
    }
}
