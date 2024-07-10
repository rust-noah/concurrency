use anyhow::Result;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// 读写不分离的例子, 频繁读和写

fn main() -> Result<()> {
    let stock_data = Arc::new(RwLock::new(StockData {
        price: 100.0,
        volume: 1000,
    }));

    // 模拟实时写操作，更新股票价格
    let writer = {
        // let stock_data = Arc::clone(&stock_data);
        let stock_data = stock_data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                {
                    let mut data = stock_data.write().unwrap();
                    data.price += 1.0;
                    data.volume += 100;
                    println!(
                        "Updated stock data: price={}, volume={}",
                        data.price, data.volume
                    );
                }
                thread::sleep(Duration::from_millis(100));
            }
        })
    };

    // 模拟多个读取操作，获取最新的股票价格
    let readers: Vec<_> = (0..5)
        .map(|i| {
            // let stock_data = Arc::clone(&stock_data);
            let stock_data = stock_data.clone();
            thread::spawn(move || {
                for _ in 0..10 {
                    {
                        let data = stock_data.read().unwrap();
                        println!("Reader {}: price={}, volume={}", i, data.price, data.volume);
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

struct StockData {
    price: f64,
    volume: u64,
}
