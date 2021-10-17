use std::net::Ipv4Addr;

use anyhow::Result;
use simple_logger::SimpleLogger;
use vectormq_client::{Vmq, API};

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut v = Vmq::new(Ipv4Addr::LOCALHOST, 5555, "queue".to_string()).await?;
    v.subscribe(Box::new(callback)).await?;

    Ok(())
}

fn callback(s: String) {
    println!("Message {}", s);
}
