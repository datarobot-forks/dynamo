// SPDX-FileCopyrightText: Copyright (c) 2024-2025 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
use lazy_static::lazy_static;
use log::info;
use std::env;
mod model_manager;
pub use model_manager::{ModelManager, ModelManagerError};

mod model_entry;
pub use model_entry::ModelEntry;

mod watcher;
pub use watcher::ModelWatcher;

lazy_static! {
    /// The root etcd path for ModelEntry, initialized from the
    /// "MODEL_ROOT_PATH" environment variable, falling back to "models".
    pub static ref MODEL_ROOT_PATH: String = {
        let path = env::var("MODEL_ROOT_PATH")
            .unwrap_or_else(|_| "models".to_string());

        // ✨ Logging statement added here ✨
        info!("Model root path set to: '{}'", path);

        path
    };
}
