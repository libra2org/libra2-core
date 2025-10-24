// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::module_and_script_storage::module_storage::Libra2ModuleStorage;
use move_binary_format::file_format::CompiledScript;
use move_vm_runtime::Script;
use move_vm_types::code::ScriptCache;

/// Represents code storage used by the Libra2 blockchain, capable of caching scripts and modules.
pub trait Libra2CodeStorage:
    Libra2ModuleStorage + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}

impl<T> Libra2CodeStorage for T where
    T: Libra2ModuleStorage
        + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}
