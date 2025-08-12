module poc::zero_internal {
    use libra2_std::crypto_algebra::{Self, Element};
    use libra2_std::bls12381_algebra::{Fr};

    public entry fun main(_owner: &signer) {
        let _fr_zero: Element<Fr> = crypto_algebra::zero<Fr>();
    }

    #[test(owner=@0x123)]
    fun a(owner: &signer){
        main(owner);
    }
}
