# libra2-ledger

Provides a set of commands and methods to communicate with Aptos app on Ledger

The methods supported by the libra2-ledger are:

- `get_app_version()`
- `get_app_name()`
- `get_public_key()`
- `sign_txn()`
- get_public_key()
- sign_txn()

NOTE: All methods and commands requires the user to have Aptos ledger app unlocked and open

## Examples

### Get Public Key from your Aptos account on Ledger

```rust
use libra2_ledger::get_public_key;

let public_key = match get_public_key(false);
println!("Public Key: {:?}", public_key);
// "Public Key: ______"
```

### Sign Transaction

Currently we only support derivative path at index 0, and the transaction has to be serialized using BCS format

```rust
use libra2_ledger::sign_txn;
use bcs;

let utf8_str = "my transaction";
let signed_txn = sign_txn(to_bytes(utf8_str)?);
```
