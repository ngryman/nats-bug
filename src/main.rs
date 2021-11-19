use anyhow::Result;
use nats::asynk::Subscription;
use nats_test_server::NatsTestServer;

mod test {
  use super::*;

  pub(super) async fn success() -> Result<()> {
    let nats = NatsTestServer::build().spawn();

    // Subscribe
    let nc = nats::asynk::connect(&nats.address().to_string()).await?;
    let sub = nc.subscribe("test").await?;

    // Publish
    let nc = nats::asynk::connect(&nats.address().to_string()).await?;
    nc.publish("test", "foo").await?;

    // Assert
    let msg = sub.next().await.unwrap();
    assert_eq!(String::from_utf8_lossy(&msg.data), "foo");

    Ok(())
  }

  pub(super) async fn timeout() -> Result<()> {
    let nats = NatsTestServer::build().spawn();

    // Subscribe
    let sub = timeout::subscribe(&nats.address().to_string(), "test").await?;

    // Publish
    let nc = nats::asynk::connect(&nats.address().to_string()).await?;
    nc.publish("test", "foo").await?;

    // Assert
    let msg = sub.next().await.unwrap();
    assert_eq!(String::from_utf8_lossy(&msg.data), "foo");

    Ok(())
  }

  mod timeout {
    use super::*;

    pub(super) async fn subscribe(addr: &str, subject: &str) -> Result<Subscription> {
      let nc = nats::asynk::connect(addr).await?;
      let sub = nc.subscribe(subject).await?;
      Ok(sub)
    }
  }
}

#[async_std::main]
async fn main() -> Result<()> {
  test::success().await?;
  test::timeout().await?;
  Ok(())
}
