use anyhow::anyhow;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaClient {
    pub producer: Arc<FutureProducer>,
}

impl KafkaClient {
    pub fn new(brokers: &str) -> KafkaClient {
        let mut creator = ClientConfig::new();
        tracing::info!("conneting to: {:?}", brokers);
        creator.set("bootstrap.servers", brokers);
        // creator.set("message.send.max.retries", "3");
        // creator.set("message.timeout.ms", "5000");
        // creator.set("queue.buffering.max.ms", "5000");
        // creator.set("queue.buffering.max.kbytes", "32768");
        // creator.set("batch.num.messages", "200");
        let producer: FutureProducer = creator.create().expect("Producer creation error");
        KafkaClient {
            producer: Arc::new(producer),
        }
    }

    pub async fn send_text(&self, topic_name: &str, data: &str, key: String) -> anyhow::Result<()> {
        let _delivery_status = self
            .producer
            .send(
                FutureRecord::to(topic_name)
                    .payload(data)
                    .key(&format!("{}", key))
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "header_key",
                        value: Some("header_value"),
                    })),
                Duration::from_secs(0),
            )
            .await
            .map_err(|e| {
                return Err::<(), anyhow::Error>(anyhow!("send kafka msg with error: {:?}", e));
            })
            .unwrap();
        anyhow::Ok(())
    }

    pub async fn send_binary(
        &self,
        topic_name: &str,
        data: &Vec<u8>,
        key: String,
    ) -> anyhow::Result<()> {
        let _delivery_status = self
            .producer
            .send(
                FutureRecord::to(topic_name)
                    .payload(data)
                    .key(&format!("{}", key))
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "header_key",
                        value: Some("header_value"),
                    })),
                Duration::from_secs(0),
            )
            .await
            .map_err(|e| {
                return Err::<(), anyhow::Error>(anyhow!("send kafka msg with error: {:?}", e));
            })
            .unwrap();
        anyhow::Ok(())
    }
}
