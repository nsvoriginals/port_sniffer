use futures::stream::{FuturesUnordered, StreamExt};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

const TIMEOUT_MS: u64 = 500;
const MAX_CONCURRENT: usize = 500;

pub async fn scan_ports(addr: IpAddr, start: u16, end: u16) -> Vec<u16> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let mut tasks = FuturesUnordered::new();
    for port in start..=end {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let address = addr;
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let result = timeout(
                Duration::from_millis(TIMEOUT_MS),
                TcpStream::connect((address, port)),
            )
            .await;

            match result {
                Ok(Ok(_)) => Some(port),
                _ => None,
            }
        }));
    }

    let mut open_ports = Vec::new();
    while let Some(Ok(Some(port))) = tasks.next().await {
        open_ports.push(port);
    }

    open_ports.sort();
    open_ports
}
