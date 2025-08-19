// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod basic_processor;
pub mod models;
pub mod subconfigs;
pub mod utils;

#[path = "db/processor_metadata_schema.rs"]
pub mod processor_metadata_schema;

pub const SDK_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/postgres/db/migrations");
