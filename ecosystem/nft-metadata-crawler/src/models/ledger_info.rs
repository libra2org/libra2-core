// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::schema::nft_metadata_crawler::ledger_infos;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

#[derive(Debug, Identifiable, Insertable, Queryable)]
#[diesel(table_name = ledger_infos)]
#[diesel(primary_key(chain_id))]
pub struct LedgerInfo {
    pub chain_id: i64,
}

impl LedgerInfo {
    pub fn get(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> diesel::QueryResult<Option<Self>> {
        ledger_infos::table
            .select(ledger_infos::all_columns)
            .first::<Self>(conn)
            .optional()
    }
}
