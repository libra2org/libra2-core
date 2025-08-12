//# init --addresses Alice=0xf75daa73fc071f93593335eb9033da804777eb94491650dd3f095ce6f778acb6
//#      --private-keys Alice=56a26140eb233750cd14fb168c3eb4bd0782b099cde626ec8aff7f3cceb6364f

//# publish --private-key 56a26140eb233750cd14fb168c3eb4bd0782b099cde626ec8aff7f3cceb6364f
module Alice::hello_world {
    use libra2_framework::signer;
    use libra2_framework::coin;
    use libra2_framework::libra2_coin::Libra2Coin;
    use std::string::{Self, String};

    struct ModuleData has key, store {
        global_counter: u64,
        state: String,
    }

    public entry fun  initialize(sender: &signer) {
        move_to(
            sender,
            ModuleData { global_counter: 0, state: string::utf8(b"init") }
        );
    }

    public fun foo(addr: address): u64 {
        coin::balance<Libra2Coin>(addr)
    }

    public entry fun hi(sender: &signer, msg: String) acquires ModuleData {
        borrow_global_mut<ModuleData>(signer::address_of(sender)).state = msg;
    }

    public entry fun hi_abort(sender: &signer) {
        assert!(!exists<ModuleData>(signer::address_of(sender)), 12);
    }
}

//# run --signers Alice --show-events -- Alice::hello_world::initialize

//# run --signers Alice --args x"68656C6C6F20776F726C64" --show-events -- Alice::hello_world::hi

//# view --address Alice --resource Alice::hello_world::ModuleData

//# run --signers Alice -- Alice::hello_world::hi_abort
