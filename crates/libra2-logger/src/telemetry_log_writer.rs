// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::counters::{libra2_log_ingest_writer_disconnected, libra2_log_ingest_writer_full};
use futures::channel;
use std::{
    io::{Error, ErrorKind},
    sync,
};

#[derive(Debug)]
pub enum TelemetryLog {
    Log(String),
    Flush(sync::mpsc::SyncSender<()>),
}

#[derive(Debug)]
pub(crate) struct TelemetryLogWriter {
    tx: channel::mpsc::Sender<TelemetryLog>,
}

impl TelemetryLogWriter {
    pub fn new(tx: channel::mpsc::Sender<TelemetryLog>) -> Self {
        Self { tx }
    }
}

impl TelemetryLogWriter {
    pub fn write(&mut self, log: String) -> std::io::Result<usize> {
        let len = log.len();
        match self.tx.try_send(TelemetryLog::Log(log)) {
            Ok(_) => Ok(len),
            Err(err) => {
                if err.is_full() {
                    libra2_log_ingest_writer_full.inc_by(len as u64);
                    Err(Error::new(ErrorKind::WouldBlock, "Channel full"))
                } else {
                    libra2_log_ingest_writer_disconnected.inc_by(len as u64);
                    Err(Error::new(ErrorKind::ConnectionRefused, "Disconnected"))
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn flush(&mut self) -> std::io::Result<sync::mpsc::Receiver<()>> {
        let (tx, rx) = sync::mpsc::sync_channel(1);
        match self.tx.try_send(TelemetryLog::Flush(tx)) {
            Ok(_) => Ok(rx),
            Err(err) => {
                if err.is_full() {
                    Err(Error::new(ErrorKind::WouldBlock, "Channel full"))
                } else {
                    Err(Error::new(ErrorKind::ConnectionRefused, "Disconnected"))
                }
            },
        }
    }
}
