use std::sync::Arc;

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub indices: Arc<Vec<usize>>,
    pub next_index: usize,
    pub snap_user_requested: bool,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            indices: Arc::new(Vec::new()),
            next_index: 0,
            snap_user_requested: true,
        }
    }
}
