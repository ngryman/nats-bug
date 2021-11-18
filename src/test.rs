#[cfg(test)]
mod test {
  #[async_std::test]
  async fn success() {
    // Arrange
    let nats = NatsTestServer::build().spawn();
    let sub = utils::subscribe(&nats.address().to_string(), "authorizer.test")
      .await
      .unwrap();
    let broker = Broker::connect(&nats.address().to_string()).await.unwrap();

    // Act
    broker
      .publish(&utils::TestEvent { id: 1337 })
      .await
      .unwrap();

    // Assert
    let msg = sub.next().await.unwrap();
    assert_eq!(
      bincode::deserialize::<utils::TestEvent>(&msg.data).unwrap(),
      utils::TestEvent { id: 1337 }
    );
  }

  mod utils {
    pub(super) async fn subscribe(nats: &NatsTestServer, subject: &str) -> Result<Subscription> {
      let nc = async_nats::connect(addr).await?;
      let sub = nc.subscribe(subject).await?;
      Ok(sub)
    }
  }
}
