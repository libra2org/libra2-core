// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

//! AI fast-path scaffold for Libra2 consensus (feature-gated).
//! - ONNX Runtime 0.0.14-compatible wrapper to score validator responsiveness.
//! - Deterministic feature normalization helpers.
//! - Safety: this does NOT change QC threshold math.

use anyhow::*;
use std::{path::Path, time::SystemTime};

#[allow(unused_imports)]
use ndarray::prelude::*;
#[allow(unused_imports)]
use onnxruntime::{environment::Environment, session::Session, GraphOptimizationLevel};

/// Config for AI responsiveness scoring.
#[derive(Clone, Debug)]
pub struct AIConfig {
    pub model_path: String,
    /// r = clamp(1 - beta * p_choke, r_min, 1)
    pub beta: f32,
    pub r_min: f32,
}

/// Lightweight predictor wrapper around ONNX Runtime.
/// NOTE: To honor `#![forbid(unsafe_code)]` and avoid lifetime issues in `onnxruntime 0.0.14`,
/// we construct an Environment + Session *per inference* instead of storing a long-lived Session.
pub struct AIPredictor {
    beta: f32,
    r_min: f32,
    model_mtime: Option<SystemTime>,
    model_path: String,
}

impl AIPredictor {
    /// Initialize predictor (verifies model file exists and captures mtime).
    pub fn load<P: AsRef<Path>>(model: P, beta: f32, r_min: f32) -> Result<Self> {
        let model_path = model.as_ref().to_string_lossy().to_string();
        // Capture initial mtime if available
        let mtime = std::fs::metadata(&model_path).ok().and_then(|m| m.modified().ok());
        Ok(Self {
            beta,
            r_min,
            model_mtime: mtime,
            model_path,
        })
    }

    /// Hot-reload metadata if the onnx file changed on disk (no-op if unchanged).
    pub fn try_hot_reload(&mut self) -> Result<bool> {
        let new_mtime = std::fs::metadata(&self.model_path)
            .ok()
            .and_then(|m| m.modified().ok());
        let changed = match (self.model_mtime, new_mtime) {
            (Some(old), Some(new)) => old < new,
            (None, Some(_)) => true,
            _ => false,
        };
        if changed {
            self.model_mtime = new_mtime;
        }
        Ok(changed)
    }

    /// Compute responsiveness factor r ∈ [r_min, 1].
    ///
    /// Input features (pre-normalization):
    /// - last_vote_delay_sec       in [0..3] (clip)
    /// - avg_ping_ms               in [10..1000] (clip)
    /// - cpu_load                  in [0..1]
    /// - stake_fraction            in [0..1]
    /// - past_lag_count100         in [0..100]
    pub fn responsiveness(&self, features: [f32; 5]) -> Result<f32> {
        // Build a fresh environment and session (keeps code safe & simple for 0.0.14).
        let env = Environment::builder().with_name("libra2-ai").build()?;
        let mut sess: Session<'_> = env
            .new_session_builder()?
            .with_optimization_level(GraphOptimizationLevel::Basic)?
            .with_model_from_file(&self.model_path)?;

        // 1x5 input
        let input: Array2<f32> = Array2::from_shape_vec((1, 5), features.to_vec())?;
        // Run session
        let mut outputs = sess.run(vec![input])?;

        // Expected scalar output (1x1). Extract first element safely.
        let p_choke: f32 = outputs
            .pop()
            .and_then(|t| t.as_slice().and_then(|s| s.first().copied()))
            .unwrap_or(0.0_f32)
            .max(0.0_f32)
            .min(1.0_f32);

        let r = (1.0_f32 - self.beta * p_choke).max(self.r_min).min(1.0_f32);
        Ok(r)
    }
}

/// Normalize raw features into [0,1] range deterministically.
pub fn normalize_features(
    last_vote_delay_sec: f32, // clip 0..3s
    avg_ping_ms: f32,         // clip 10..1000
    cpu_load: f32,            // 0..1
    stake_fraction: f32,      // 0..1
    past_lag_count100: f32,   // 0..100
) -> [f32; 5] {
    fn norm(x: f32, lo: f32, hi: f32) -> f32 {
        ((x - lo) / (hi - lo)).clamp(0.0, 1.0)
    }
    [
        norm(last_vote_delay_sec.clamp(0.0, 3.0), 0.0, 3.0),
        norm(avg_ping_ms.clamp(10.0, 1000.0), 10.0, 1000.0),
        cpu_load.clamp(0.0, 1.0),
        stake_fraction.clamp(0.0, 1.0),
        (past_lag_count100 / 100.0).clamp(0.0, 1.0),
    ]
}

/// Safe default: if AI is temporarily unavailable, return r=1.0
pub fn safe_default_r() -> f32 { 1.0 }
