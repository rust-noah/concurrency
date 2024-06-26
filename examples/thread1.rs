use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

// region:    --- impls
impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
// endregion: --- impls

// region:    --- functions
fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>(); // usize on 64-bit platform is u64
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}
// endregion: --- functions

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone(); // use mpsc model, tx can be clone many times
        thread::spawn(move || producer(i, tx)); // use move -> closures need ownership, not references
    }
    // Because an extra tx was created, including the one at the top,
    // there is an extra copy here, so it needs to be manually dropped.
    drop(tx);

    // create consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });
    let secret = consumer
        .join() // Let the main thread actively wait for the child thread to finish.
        .map_err(|e| anyhow!("thread join error: {:?}", e))?;
    println!("secret: {}", secret);
    Ok(())
}
