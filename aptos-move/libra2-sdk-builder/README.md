---
id: libra2-sdk-builder
title: Libra2 SDK Builder
custom_edit_url: https://github.com/libra2org/libra2-core/edit/main/language/libra2-sdk-builder/README.md
---

# Libra2 SDK Builder

A *transaction builder* is a helper function that converts its arguments into the payload of an Libra2 transaction calling a particular Move script.

In Rust, the signature of such a function typically looks like this:
```rust
pub fn encode_peer_to_peer_with_metadata_script(
    token: TypeTag,
    payee: AccountAddress,
    amount: u64,
    metadata: Vec<u8>,
    metadata_signature: Vec<u8>,
) -> Script;
```

This crate provide a library to generate transaction builders in one programming language.

The tool will also generate and install type definitions for Libra2 types such as `TypeTag`, `AccountAddress`, and `Script`.

In practice, hashing and signing Libra2 transactions additionally requires a runtime library for Binary Canonical Serialization ("BCS").
Such a library will be installed together with the Libra2 types.


## Supported Languages

The following languages are currently supported:
* Rust
