extern crate qp2p;
extern crate anyhow;

use qp2p::{Config, ConnId, Endpoint};
use anyhow::Result;
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

pub fn connect(queue_name: String) -> Result<()> {

    let (node, _incoming_conns, mut _incoming_messages, _disconnections, _contact) =
    Endpoint::new(
        SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
        &[],
        Config {
            idle_timeout: Duration::from_secs(60 * 60).into(), // 1 hour idle timeout.
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}