use std::sync::Arc;

use tracing::info;

pub struct Controller {
    _data: Arc<()>,
}

impl Controller {
    pub fn new(_data: Arc<()>) -> Self {
        Controller { _data }
    }

    pub fn start(&self) {
        info!("let's start this thing up!");
    }
}
