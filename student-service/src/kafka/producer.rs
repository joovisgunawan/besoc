use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use std::time::Duration;
use tracing::instrument;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(broker: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .create()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to create Kafka producer: {e}");
                std::process::exit(1);
            });

        Self { producer }
    }

    #[instrument(skip(self, payload), fields(kafka.topic = topic, kafka.key = key))]
    pub async fn publish(&self, topic: &str, key: &str, payload: &str) -> Result<(), String> {
        self.producer
            .send(
                FutureRecord::to(topic).key(key).payload(payload),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| e.to_string())?;

        tracing::info!("Event published");
        Ok(())
    }
}