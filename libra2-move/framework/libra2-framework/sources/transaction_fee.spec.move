spec libra2_framework::transaction_fee {
    /// <high-level-req>
    /// No.: 1
    /// Requirement: Given the blockchain is in an operating state, it guarantees that the Aptos framework signer may burn
    /// Aptos coins.
    /// Criticality: Critical
    /// Implementation: The Libra2CoinCapabilities structure is defined in this module and it stores burn capability to
    /// burn the gas fees.
    /// Enforcement: Formally Verified via [high-level-req-1](module).
    ///
    /// No.: 2
    /// Requirement: The initialization function may only be called once.
    /// Criticality: Medium
    /// Implementation: The initialize_fee_collection_and_distribution function ensures CollectedFeesPerBlock does not
    /// already exist.
    /// Enforcement: Formally verified via [high-level-req-2](initialize_fee_collection_and_distribution).
    ///
    /// No.: 3
    /// Requirement: Only the admin address is authorized to call the initialization function.
    /// Criticality: Critical
    /// Implementation: The initialize_fee_collection_and_distribution function ensures only the Aptos framework address
    /// calls it.
    /// Enforcement: Formally verified via [high-level-req-3](initialize_fee_collection_and_distribution).
    ///
    /// No.: 4
    /// Requirement: The percentage of the burnt collected fee is always a value from 0 to 100.
    /// Criticality: Medium
    /// Implementation: During the initialization of CollectedFeesPerBlock in
    /// Initialize_fee_collection_and_distribution, and while upgrading burn percentage, it asserts that burn_percentage
    /// is within the specified limits.
    /// Enforcement: Formally verified via [high-level-req-4](CollectedFeesPerBlock).
    ///
    /// No.: 5
    /// Requirement: Prior to upgrading the burn percentage, it must process all the fees collected up to that point.
    /// Criticality: Critical
    /// Implementation: The upgrade_burn_percentage function ensures process_collected_fees function is called before
    /// updating the burn percentage.
    /// Enforcement: Formally verified in [high-level-req-5](ProcessCollectedFeesRequiresAndEnsures).
    ///
    /// No.: 6
    /// Requirement: The presence of the resource, indicating collected fees per block under the Aptos framework account,
    /// is a prerequisite for the successful execution of the following functionalities: Upgrading burn percentage.
    /// Registering a block proposer. Processing collected fees.
    /// Criticality: Low
    /// Implementation: The functions: upgrade_burn_percentage, register_proposer_for_fee_collection, and
    /// process_collected_fees all ensure that the CollectedFeesPerBlock resource exists under libra2_framework by
    /// calling the is_fees_collection_enabled method, which returns a boolean value confirming if the resource exists
    /// or not.
    /// Enforcement: Formally verified via [high-level-req-6.1](register_proposer_for_fee_collection), [high-level-req-6.2](process_collected_fees), and [high-level-req-6.3](upgrade_burn_percentage).
    /// </high-level-req>
    ///
    spec module {
        use libra2_framework::chain_status;

        // TODO(fa_migration)
        pragma verify = false;

        pragma aborts_if_is_strict;
        // property 1: Given the blockchain is in an operating state, it guarantees that the Aptos framework signer may burn Aptos coins.
        /// [high-level-req-1]
        invariant [suspendable] chain_status::is_operating() ==> exists<Libra2CoinCapabilities>(@libra2_framework) || exists<AptosFABurnCapabilities>(@libra2_framework);
    }

    spec CollectedFeesPerBlock {
        // property 4: The percentage of the burnt collected fee is always a value from 0 to 100.
        /// [high-level-req-4]
        invariant burn_percentage <= 100;
    }

    spec initialize_fee_collection_and_distribution(_libra2_framework: &signer, _burn_percentage: u8) {
    }

    /// `Libra2CoinCapabilities` should be exists.
    spec burn_fee(account: address, fee: u64) {
        use aptos_std::type_info;
        use libra2_framework::optional_aggregator;
        use libra2_framework::coin;
        use libra2_framework::coin::{CoinInfo, CoinStore};
        // TODO(fa_migration)
        pragma verify = false;

        aborts_if !exists<Libra2CoinCapabilities>(@libra2_framework);

        // This function essentially calls `coin::burn_coin`, monophormized for `Libra2Coin`.
        let account_addr = account;
        let amount = fee;

        let aptos_addr = type_info::type_of<Libra2Coin>().account_address;
        let coin_store = global<CoinStore<Libra2Coin>>(account_addr);
        let post post_coin_store = global<CoinStore<Libra2Coin>>(account_addr);

        // modifies global<CoinStore<Libra2Coin>>(account_addr);

        aborts_if amount != 0 && !(exists<CoinInfo<Libra2Coin>>(aptos_addr)
            && exists<CoinStore<Libra2Coin>>(account_addr));
        aborts_if coin_store.coin.value < amount;

        let maybe_supply = global<CoinInfo<Libra2Coin>>(aptos_addr).supply;
        let supply_aggr = option::spec_borrow(maybe_supply);
        let value = optional_aggregator::optional_aggregator_value(supply_aggr);

        let post post_maybe_supply = global<CoinInfo<Libra2Coin>>(aptos_addr).supply;
        let post post_supply = option::spec_borrow(post_maybe_supply);
        let post post_value = optional_aggregator::optional_aggregator_value(post_supply);

        aborts_if option::spec_is_some(maybe_supply) && value < amount;

        ensures post_coin_store.coin.value == coin_store.coin.value - amount;
        ensures if (option::spec_is_some(maybe_supply)) {
            post_value == value - amount
        } else {
            option::spec_is_none(post_maybe_supply)
        };
        ensures coin::supply<Libra2Coin> == old(coin::supply<Libra2Coin>) - amount;
    }

    spec mint_and_refund(account: address, refund: u64) {
        use aptos_std::type_info;
        use libra2_framework::libra2_coin::Libra2Coin;
        use libra2_framework::coin::{CoinInfo, CoinStore};
        use libra2_framework::coin;
        // TODO(fa_migration)
        pragma verify = false;
        // pragma opaque;

        let aptos_addr = type_info::type_of<Libra2Coin>().account_address;

        aborts_if (refund != 0) && !exists<CoinInfo<Libra2Coin>>(aptos_addr);
        include coin::CoinAddAbortsIf<Libra2Coin> { amount: refund };

        aborts_if !exists<CoinStore<Libra2Coin>>(account);
        // modifies global<CoinStore<Libra2Coin>>(account);

        aborts_if !exists<Libra2CoinMintCapability>(@libra2_framework);

        let supply = coin::supply<Libra2Coin>;
        let post post_supply = coin::supply<Libra2Coin>;
        aborts_if [abstract] supply + refund > MAX_U128;
        ensures post_supply == supply + refund;
    }

    /// Ensure caller is admin.
    /// Aborts if `Libra2CoinCapabilities` already exists.
    spec store_libra2_coin_burn_cap(libra2_framework: &signer, burn_cap: BurnCapability<Libra2Coin>) {
        use std::signer;

        // TODO(fa_migration)
        pragma verify = false;

        let addr = signer::address_of(libra2_framework);
        aborts_if !system_addresses::is_libra2_framework_address(addr);

        aborts_if exists<AptosFABurnCapabilities>(addr);
        aborts_if exists<Libra2CoinCapabilities>(addr);

        ensures exists<AptosFABurnCapabilities>(addr) || exists<Libra2CoinCapabilities>(addr);
    }

    /// Ensure caller is admin.
    /// Aborts if `Libra2CoinMintCapability` already exists.
    spec store_libra2_coin_mint_cap(libra2_framework: &signer, mint_cap: MintCapability<Libra2Coin>) {
        use std::signer;
        let addr = signer::address_of(libra2_framework);
        aborts_if !system_addresses::is_libra2_framework_address(addr);
        aborts_if exists<Libra2CoinMintCapability>(addr);
        ensures exists<Libra2CoinMintCapability>(addr);
    }

    /// Historical. Aborts.
    spec initialize_storage_refund(_: &signer) {
        aborts_if true;
    }

    /// Aborts if module event feature is not enabled.
    spec emit_fee_statement {}
}
