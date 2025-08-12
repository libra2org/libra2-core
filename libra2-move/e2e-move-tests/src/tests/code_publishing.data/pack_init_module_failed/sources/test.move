module 0xcafe::test {
    use libra2_framework::coin::{Self, Coin};
    use libra2_framework::libra2_coin::Libra2Coin;
    use std::signer::address_of;

    struct State has key {
        important_value: u64,
        coins: Coin<Libra2Coin>,
    }

    fun init_module(s: &signer) {
        // Transfer away all the APT from s so there's nothing left to pay for gas.
        // This makes this init_module function fail for sure.
        let balance = coin::balance<Libra2Coin>(address_of(s));
        let coins = coin::withdraw<Libra2Coin>(s, balance);

        move_to(s, State {
            important_value: get_value(),
            coins,
        })
    }

    fun get_value(): u64 {
        1
    }
}
