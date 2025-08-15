// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// @generated
impl serde::Serialize for ChainIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("libra2.txnstream.v1.ChainIdResponse", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.ChainIdResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChainIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChainIdResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.ChainIdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Empty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("libra2.txnstream.v1.Empty", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Empty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Empty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.Empty")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Empty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Empty {
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.Empty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.build_timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("libra2.txnstream.v1.ServerInfoResponse", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.build_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("buildTimestamp", ToString::to_string(&self.build_timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "build_timestamp",
            "buildTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            BuildTimestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "buildTimestamp" | "build_timestamp" => Ok(GeneratedField::BuildTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.ServerInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut build_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuildTimestamp => {
                            if build_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildTimestamp"));
                            }
                            build_timestamp__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ServerInfoResponse {
                    version: version__.unwrap_or_default(),
                    build_timestamp: build_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.ServerInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.txn.is_empty() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("libra2.txnstream.v1.TransactionOutput", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if !self.txn.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("txn", pbjson::private::base64::encode(&self.txn).as_str())?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.info.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("info", pbjson::private::base64::encode(&self.info).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "txn",
            "events",
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Txn,
            Events,
            Info,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "txn" => Ok(GeneratedField::Txn),
                            "events" => Ok(GeneratedField::Events),
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.TransactionOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransactionOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut txn__ = None;
                let mut events__ = None;
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Txn => {
                            if txn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txn"));
                            }
                            txn__ =
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ =
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ =
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TransactionOutput {
                    version: version__.unwrap_or_default(),
                    txn: txn__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                    info: info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.TransactionOutput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_version != 0 {
            len += 1;
        }
        if self.include_events {
            len += 1;
        }
        if self.batch_size != 0 {
            len += 1;
        }
        if self.max_wait_ms != 0 {
            len += 1;
        }
        if self.exclude_ledger_changes {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("libra2.txnstream.v1.TransactionsRequest", len)?;
        if self.start_version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("startVersion", ToString::to_string(&self.start_version).as_str())?;
        }
        if self.include_events {
            struct_ser.serialize_field("includeEvents", &self.include_events)?;
        }
        if self.batch_size != 0 {
            struct_ser.serialize_field("batchSize", &self.batch_size)?;
        }
        if self.max_wait_ms != 0 {
            struct_ser.serialize_field("maxWaitMs", &self.max_wait_ms)?;
        }
        if self.exclude_ledger_changes {
            struct_ser.serialize_field("excludeLedgerChanges", &self.exclude_ledger_changes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_version",
            "startVersion",
            "include_events",
            "includeEvents",
            "batch_size",
            "batchSize",
            "max_wait_ms",
            "maxWaitMs",
            "exclude_ledger_changes",
            "excludeLedgerChanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartVersion,
            IncludeEvents,
            BatchSize,
            MaxWaitMs,
            ExcludeLedgerChanges,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "startVersion" | "start_version" => Ok(GeneratedField::StartVersion),
                            "includeEvents" | "include_events" => Ok(GeneratedField::IncludeEvents),
                            "batchSize" | "batch_size" => Ok(GeneratedField::BatchSize),
                            "maxWaitMs" | "max_wait_ms" => Ok(GeneratedField::MaxWaitMs),
                            "excludeLedgerChanges" | "exclude_ledger_changes" => Ok(GeneratedField::ExcludeLedgerChanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.TransactionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransactionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_version__ = None;
                let mut include_events__ = None;
                let mut batch_size__ = None;
                let mut max_wait_ms__ = None;
                let mut exclude_ledger_changes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartVersion => {
                            if start_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startVersion"));
                            }
                            start_version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IncludeEvents => {
                            if include_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeEvents"));
                            }
                            include_events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchSize => {
                            if batch_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchSize"));
                            }
                            batch_size__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxWaitMs => {
                            if max_wait_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxWaitMs"));
                            }
                            max_wait_ms__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExcludeLedgerChanges => {
                            if exclude_ledger_changes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludeLedgerChanges"));
                            }
                            exclude_ledger_changes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TransactionsRequest {
                    start_version: start_version__.unwrap_or_default(),
                    include_events: include_events__.unwrap_or_default(),
                    batch_size: batch_size__.unwrap_or_default(),
                    max_wait_ms: max_wait_ms__.unwrap_or_default(),
                    exclude_ledger_changes: exclude_ledger_changes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.TransactionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batch.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("libra2.txnstream.v1.TransactionsResponse", len)?;
        if !self.batch.is_empty() {
            struct_ser.serialize_field("batch", &self.batch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "batch" => Ok(GeneratedField::Batch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct libra2.txnstream.v1.TransactionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransactionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TransactionsResponse {
                    batch: batch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("libra2.txnstream.v1.TransactionsResponse", FIELDS, GeneratedVisitor)
    }
}
