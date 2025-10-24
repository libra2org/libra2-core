// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum Ref<T> {
    Strong(Arc<T>),
    Weak(Weak<T>),
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Strong(arc) => Self::Strong(arc.clone()),
            Self::Weak(weak) => Self::Weak(weak.clone()),
        }
    }
}

impl<T> Ref<T> {
    pub fn try_get_strong(&self) -> Option<Arc<T>> {
        match self {
            Self::Strong(arc) => Some(arc.clone()),
            Self::Weak(weak) => weak.upgrade(),
        }
    }
}
