use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// 假设我们有一个网站访问计数器，它统计每个页面的访问次数。网站有大量的读取操作，因为每个访问者都会查看访问次数，
// 但只有少量的写入操作，因为只有在访问页面时才会增加访问次数。

fn main() -> Result<()> {
    let page_views = Arc::new(RwLock::new(HashMap::new()));

    // 模拟写操作，增加页面访问次数
    let writer = {
        // let page_views = Arc::clone(&page_views);
        let page_views = page_views.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                {
                    let mut views = page_views.write().unwrap();
                    let counter = views.entry("home".to_string()).or_insert(0);
                    *counter += 1;
                    println!("Page 'home' visited {} times", *counter);
                }
                thread::sleep(Duration::from_millis(100));
            }
        })
    };

    // 模拟多个读取操作，查看页面访问次数
    let readers: Vec<_> = (0..5)
        .map(|i| {
            // let page_views = Arc::clone(&page_views);
            let page_views = page_views.clone();
            thread::spawn(move || {
                for _ in 0..10 {
                    {
                        let views = page_views.read().unwrap();
                        let counter = views.get("home").unwrap_or(&0);
                        println!("Reader {}: Page 'home' visited {} times", i, counter);
                    }
                    thread::sleep(Duration::from_millis(50));
                }
            })
        })
        .collect();

    writer.join().unwrap();
    for reader in readers {
        reader.join().unwrap();
    }
    Ok(())
}
