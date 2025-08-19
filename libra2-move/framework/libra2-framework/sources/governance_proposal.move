/// Define the GovernanceProposal that will be used as part of on-chain governance by Libra2Governance.
///
/// This is separate from the Libra2Governance module to avoid circular dependency between Libra2Governance and Stake.
module libra2_framework::governance_proposal {
    friend libra2_framework::libra2_governance;

    struct GovernanceProposal has store, drop {}

    /// Create and return a GovernanceProposal resource. Can only be called by Libra2Governance
    public(friend) fun create_proposal(): GovernanceProposal {
        GovernanceProposal {}
    }

    /// Useful for Libra2Governance to create an empty proposal as proof.
    public(friend) fun create_empty_proposal(): GovernanceProposal {
        create_proposal()
    }

    #[test_only]
    public fun create_test_proposal(): GovernanceProposal {
        create_empty_proposal()
    }
}
