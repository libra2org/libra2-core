script {
    use libra2_framework::libra2_governance;
    use std::features;

    fun main(core_resources: &signer, enable_partial_governance_voting: bool) {
        let framework_signer = libra2_governance::get_signer_testnet_only(core_resources, @libra2_framework);
        let feature = features::get_partial_governance_voting();
        if (enable_partial_governance_voting) {
            features::change_feature_flags_for_next_epoch(&framework_signer, vector[feature], vector[]);
        } else {
            features::change_feature_flags_for_next_epoch(&framework_signer, vector[], vector[feature]);
        };
        libra2_governance::force_end_epoch(&framework_signer);
    }
}
