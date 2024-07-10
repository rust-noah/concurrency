// metrics data structure
// basic functions: inc/dec/snapshot

use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;

// region:    --- HashMap Version
// // metrics table
// #[derive(Debug, Default, Clone)]
// pub struct Metrics {
//     // data: HashMap<String, i64>,
//     // data: Arc<Mutex<HashMap<String, i64>>>,
//     data: Arc<RwLock<HashMap<String, i64>>>,
// }

// // region:    --- impls
// impl Metrics {
//     pub fn new() -> Self {
//         // Self {
//         //     data: HashMap::new(),
//         // }
//         Self::default()
//     }
//     // pub fn inc(&mut self, key: &str) {
//     //     let counter = self.data.entry(key.to_string()).or_insert(0);
//     //     *counter += 1;
//     // }

//     // pub fn dec(&mut self, key: &str) {
//     //     let counter = self.data.entry(key.to_string()).or_insert(0);
//     //     *counter -= 1;
//     // }

//     pub fn inc(&self, key: impl Into<String>) -> Result<()> {
//         // let counter = self.data.lock.entry(key.into()).or_insert(0);
//         // *counter += 1;
//         // let mut data = self.data.lock().unwrap();
//         // let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
//         let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
//         let counter = data.entry(key.into()).or_insert(0);
//         *counter += 1;
//         Ok(())
//     }

//     pub fn dec(&self, key: impl Into<String>) -> Result<()> {
//         // let counter = self.data.entry(key.into()).or_insert(0);
//         // *counter -= 1;
//         // let mut data = self.data.lock().unwrap();
//         let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
//         let counter = data.entry(key.into()).or_insert(0);
//         *counter -= 1;
//         Ok(())
//     }

//     pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
//         // self.data.clone()
//         // Ok(self
//         //     .data
//         //     .lock()
//         //     .map_err(|e| anyhow!(e.to_string()))?
//         //     .clone())
//         Ok(self
//             .data
//             .read()
//             .map_err(|e| anyhow!(e.to_string()))?
//             .clone())
//     }
// }

// impl Display for Metrics {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let data = self.data.read().map_err(|_e| fmt::Error {})?;
//         for (key, value) in data.iter() {
//             writeln!(f, "{}: {}", key, value)?;
//         }
//         Ok(())
//     }
// }
// // endregion: --- impls
// endregion: --- HashMap Version

// region:    --- DashMap Version
#[derive(Default, Clone)]
pub struct Metrics {
    // Arc<Mutex<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
    data: Arc<DashMap<String, i64>>, // 不需要加锁, 因为 DashMap 本身是线程安全的
}

// region:    --- impls
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
// endregion: --- impls
// endregion: --- DashMap Version
