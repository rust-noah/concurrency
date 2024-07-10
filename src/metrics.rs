// metrics data structure
// basic functions: inc/dec/snapshot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};

// metrics table
#[derive(Debug, Default, Clone)]
pub struct Metrics {
    // data: HashMap<String, i64>,
    data: Arc<Mutex<HashMap<String, i64>>>,
}

// region:    --- impls
impl Metrics {
    pub fn new() -> Self {
        // Self {
        //     data: HashMap::new(),
        // }
        Self::default()
    }
    // pub fn inc(&mut self, key: &str) {
    //     let counter = self.data.entry(key.to_string()).or_insert(0);
    //     *counter += 1;
    // }

    // pub fn dec(&mut self, key: &str) {
    //     let counter = self.data.entry(key.to_string()).or_insert(0);
    //     *counter -= 1;
    // }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // let counter = self.data.lock.entry(key.into()).or_insert(0);
        // *counter += 1;
        // let mut data = self.data.lock().unwrap();
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        // let counter = self.data.entry(key.into()).or_insert(0);
        // *counter -= 1;
        let mut data = self.data.lock().unwrap();
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        // self.data.clone()
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}
// endregion: --- impls
