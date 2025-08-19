#[test_only]
module confidential_asset_example::register_example {
    use std::signer;
    use std::string::utf8;
    use libra2_std::debug::print;
    use libra2_framework::fungible_asset::Metadata;
    use libra2_framework::object::Object;

    use libra2_experimental::confidential_asset;
    use libra2_experimental::confidential_asset_tests;
    use libra2_experimental::ristretto255_twisted_elgamal as twisted_elgamal;

    fun register(bob: &signer, token: Object<Metadata>) {
        let bob_addr = signer::address_of(bob);

        // It's a test-only function, so we don't need to worry about the security of the keypair.
        let (_bob_dk, bob_ek) = twisted_elgamal::generate_twisted_elgamal_keypair();

        let bob_ek = twisted_elgamal::pubkey_to_bytes(&bob_ek);

        confidential_asset::register(bob, token, bob_ek);

        print(&utf8(b"Bob's pending balance is zero:"));
        print(&confidential_asset::pending_balance(bob_addr, token));

        print(&utf8(b"Bob's actual balance is zero:"));
        print(&confidential_asset::actual_balance(bob_addr, token));

        print(&utf8(b"Bob's encryption key is set:"));
        print(&confidential_asset::encryption_key(bob_addr, token));
    }

    #[test(
        confidential_asset = @libra2_experimental,
        libra2_fx = @libra2_framework,
        fa = @0xfa,
        bob = @0xb0
    )]
    fun register_example_test(
        confidential_asset: signer,
        libra2_fx: signer,
        fa: signer,
        bob: signer)
    {
        let token = confidential_asset_tests::set_up_for_confidential_asset_test(
            &confidential_asset,
            &libra2_fx,
            &fa,
            &bob,
            &bob,
            0,
            0
        );

        register(&bob, token);
    }
}
