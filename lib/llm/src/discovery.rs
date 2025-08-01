// SPDX-FileCopyrightText: Copyright (c) 2024-2025 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::sync::OnceLock;

mod model_manager;
pub use model_manager::{ModelManager, ModelManagerError};

mod model_entry;
pub use model_entry::ModelEntry;

mod watcher;
pub use watcher::ModelWatcher;

static MODEL_ROOT_PATH_LOCK: OnceLock<String> = OnceLock::new();

/// The root etcd path for ModelEntry
/// This is sourced from the MODEL_ROOT_PATH environment variable at runtime,
/// and defaults to "models" if it is not set.
pub fn model_root_path() -> &'static str {
    MODEL_ROOT_PATH_LOCK
        .get_or_init(|| env::var("MODEL_ROOT_PATH").unwrap_or_else(|_| "models".to_string()))
}
