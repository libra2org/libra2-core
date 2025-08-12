module libra2_experimental::benchmark_utils {
    use libra2_framework::account;
    use libra2_framework::libra2_account;

    /// Entry function that creates account resource, and funds the account.
    /// This makes sure that transactions later don't need to create an account,
    /// and so actual costs of entry functions can be more precisely measured.
    entry fun transfer_and_create_account(
        source: &signer, to: address, amount: u64
    ) {
        account::create_account_if_does_not_exist(to);
        libra2_account::transfer(source, to, amount);
    }
}
