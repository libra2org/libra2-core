// Script hash: c2035ec4
script {
    use libra2_framework::libra2_governance;
    use libra2_framework::version;

    fun main(core_resources: &signer) {
        let core_signer = libra2_governance::get_signer_testnet_only(core_resources, @0x1);

        let framework_signer = &core_signer;

        version::set_for_next_epoch(framework_signer, 999);
        libra2_governance::reconfigure(framework_signer);
    }
}
