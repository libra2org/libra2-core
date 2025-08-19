// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

#[async_trait]
pub trait ProcessorTrait: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run_processor(&self) -> anyhow::Result<()>;
}
