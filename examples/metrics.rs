use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;
// region:    --- HashMap version
// fn main() -> Result<()> {
//     let metrics = Metrics::new();
//     let start_time = Instant::now();

//     // region:    --- 单线程可用 code

//     // for i in 0..100 {
//     //     metrics.inc("req.page.1");
//     //     metrics.inc("req.page.2");
//     //     if i & 1 == 0 {
//     //         metrics.inc("req.page.3");
//     //     }
//     // }

//     // for _ in 0..27 {
//     //     metrics.inc("call.thread.worker.1");
//     // }

//     // endregion: --- 单线程可用 code

//     println!("{:?}", metrics.snapshot()?);

//     for idx in 0..N {
//         task_worker(idx, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
//     }

//     for _ in 0..M {
//         request_worker(metrics.clone())?;
//     }

//     while start_time.elapsed() < Duration::from_secs(10) {
//         thread::sleep(Duration::from_secs(2));
//         // println!("{:?}", metrics.snapshot()?); // snapshot 使用 clone 的方式
//         println!("{}", metrics); // 拿到读锁之后, 直接打印
//     }

//     Ok(())
// }
// endregion: --- HashMap version

// region:    --- DashMap version
fn main() -> Result<()> {
    let metrics = Metrics::new();
    let start_time = Instant::now();
    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    while start_time.elapsed() < Duration::from_secs(10) {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
    Ok(())
}
// endregion: --- DashMap version

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            // metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..=256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
