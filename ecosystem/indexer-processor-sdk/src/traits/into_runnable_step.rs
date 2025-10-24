// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::traits::{Processable, RunnableStep};

pub trait IntoRunnableStep<
    Input,
    Output,
    Step: Processable,
    RunnableType = <Step as Processable>::RunType,
> where
    Self: Send + Sized + 'static,
    Input: Send + 'static,
    Output: Send + 'static,
{
    fn into_runnable_step(self) -> impl RunnableStep<Input, Output>;
}
