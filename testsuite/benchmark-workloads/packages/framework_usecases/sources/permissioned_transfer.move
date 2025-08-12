
module 0xABCD::permissioned_transfer {
    use libra2_framework::libra2_account;
    use libra2_framework::permissioned_signer;
    use libra2_framework::primary_fungible_store;

    public entry fun transfer_permissioned(
        source: &signer, to: address, amount: u64
    ) {
        let handle = permissioned_signer::create_permissioned_handle(source);
        let permissioned_signer = permissioned_signer::signer_from_permissioned_handle(&handle);

        primary_fungible_store::grant_apt_permission(source, &permissioned_signer, amount);
        libra2_account::transfer(&permissioned_signer, to, amount);

        permissioned_signer::destroy_permissioned_handle(handle);
    }

    public entry fun transfer(
        source: &signer, to: address, amount: u64
    ) {
        libra2_account::transfer(source, to, amount);
    }
}
