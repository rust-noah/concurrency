use anyhow::Result;
use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

// Rust 标准库提供了一些原子类型，可以在多线程环境下安全地共享和修改数据
// 不需要使用锁，原子类型的操作是无锁的，因此性能更好
#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        // 初始化 HashMap，每个 key 对应一个 AtomicI64 类型的值
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
        counter.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}

// impl Clone for AmapMetrics {
//     fn clone(&self) -> Self {
//         AmapMetrics {
//             data: Arc::clone(&self.data),
//         }
//     }
// }

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
