use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bytes::Bytes;
use qp2p::{Config, ConnId, Connection, Endpoint, IncomingMessages};
use std::{net::*, str::from_utf8, time::Duration};

#[derive(Default, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub struct XId(pub [u8; 32]);

impl ConnId for XId {
    fn generate(_socket_addr: &SocketAddr) -> Self {
        XId(rand::random())
    }
}

const ACK: &str = "###ack###";

pub struct Vmq {
    conn: Connection<XId>,
    incoming_message: IncomingMessages,
}

impl Vmq {
    pub async fn new(url: Ipv4Addr, port: u16, queue_name: String) -> Result<Vmq> {
        let (node, _incoming_conns, incoming_messages, _disconnections, _contact) =
            Endpoint::<XId>::new(
                SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
                &[],
                Config {
                    idle_timeout: Duration::from_secs(60 * 5).into(),
                    ..Default::default()
                },
            )
            .await?;

        let conn = node.connect_to(&SocketAddr::from((url, port))).await?;
        log::info!("Connected to server");

        conn.send(Bytes::from(queue_name.clone())).await?;
        log::debug!("Connected to queue {}", queue_name);

        Ok(Vmq {
            conn,
            incoming_message: incoming_messages,
        })
    }
}

#[async_trait]
pub trait API {
    async fn subscribe(&mut self, f: Box<dyn Fn(String) + Send>) -> Result<()>;

    async fn publish(&self, message: String) -> Result<()>;
}

#[async_trait]
impl API for Vmq {
    async fn publish(&self, message: String) -> Result<()> {
        self.conn
            .send(Bytes::from(message))
            .await
            .map_err(|e| anyhow!(e))
    }

    async fn subscribe(&mut self, f: Box<dyn Fn(String) + Send>) -> Result<()> {
        ack(&self.conn).await.unwrap();

        loop {
            match &self.incoming_message.next().await {
                Some((_, message)) => {
                    f(from_utf8(&message).unwrap().to_string());
                    match ack(&self.conn).await {
                        Err(e) => {
                            log::error!("Cant consume message {}", e);
                        }
                        _ => {}
                    };
                }
                None => {}
            }
        }
    }
}

async fn ack(conn: &Connection<XId>) -> Result<()> {
    conn.send(Bytes::from(ACK))
        .await
        .map_err(|e| anyhow!("Error ack {}", e))
}
