// Copyright (c) 2025 Libra2 Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::pre_partition::{
    uniform_partitioner::UniformPartitioner, PrePartitioner, PrePartitionerConfig,
};

#[derive(Clone, Debug)]
pub struct UniformPartitionerConfig {}

impl PrePartitionerConfig for UniformPartitionerConfig {
    fn build(&self) -> Box<dyn PrePartitioner> {
        Box::new(UniformPartitioner {})
    }
}
