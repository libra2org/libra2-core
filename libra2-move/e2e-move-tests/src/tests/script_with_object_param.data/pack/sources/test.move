script {
    use libra2_framework::fungible_asset::Metadata;
    use libra2_framework::object::Object;
    use example_addr::managed_fungible_asset::transfer_between_primary_stores;

    fun main(
        admin: &signer,
        asset: Object<Metadata>,
        from: vector<address>,
        to: vector<address>,
        amounts: vector<u64>,
    ) {
        transfer_between_primary_stores(admin, asset, from, to, amounts);
    }
}
