// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

#[async_trait]
pub trait ProcessorTrait: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run_processor(&self) -> anyhow::Result<()>;
}
