use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

// redis
// 1. redis-cli (use redis-cli)
// 2. redis-server (need to implement dummy server)

// 在这个程序运行的时候, 他会先初始化一个 tokio 的运行时

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        info!("Accepted connection from: {}", remote_addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, remote_addr).await {
                warn!("Error processing conn with {}: {:?}", remote_addr, e);
            }
        });
    }
}

async fn process_redis_conn(
    mut stream: tokio::net::TcpStream,
    remote_addr: SocketAddr,
) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", remote_addr);
    Ok(())
}
