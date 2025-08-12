// Script hash: 268052ee
script {
    use libra2_framework::libra2_governance;
    use libra2_framework::jwks;

    fun main(core_resources: &signer) {
        let core_signer = libra2_governance::get_signer_testnet_only(core_resources, @0x1);

        let framework_signer = &core_signer;

        jwks::upsert_oidc_provider_for_next_epoch(framework_signer, b"https://accounts.google.com", b"https://accounts.google.com/.well-known/openid-configuration");
        jwks::remove_oidc_provider_for_next_epoch(framework_signer, b"https://www.facebook.com");
        libra2_governance::reconfigure(framework_signer);
    }
}
