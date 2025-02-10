use futures_util::StreamExt;
use rdkafka::client::ClientContext;
#[allow(unused_imports)]
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
#[allow(unused_imports)]
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::Message;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, trace, warn};

pub struct CustomContext;
impl ClientContext for CustomContext {}
impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        trace!("Committing offsets: {:?}", result);
    }
}

pub struct KafkaConsumer {}

#[derive(Debug, Clone)]
pub struct ConsumerMsg {
    pub key: Option<Vec<u8>>,
    pub payload: Option<Vec<u8>>,
    pub topic: String,
    pub timestamp: Option<i64>,
    pub partition: i32,
    pub offset: i64,
}

impl KafkaConsumer {
    pub fn start_consume(
        topic: String,
        group_id: String,
        brokers: String,
        sender: UnboundedSender<ConsumerMsg>,
    ) -> anyhow::Result<()> {
        let mut creator = ClientConfig::new();
        creator.set("group.id", group_id);
        creator.set("bootstrap.servers", brokers);
        creator.set("enable.partition.eof", "false");
        creator.set("session.timeout.ms", "6000");
        creator.set("enable.auto.commit", "false");
        // creator.set("statistics.interval.ms", "30000");
        // creator.set("auto.offset.reset", "smallest");
        // creator.set_log_level(RDKafkaLogLevel::Debug);

        let context = CustomContext;
        let consumer: StreamConsumer<CustomContext> = creator
            .create_with_context(context)
            .expect("Consumer creation failed");

        tokio::spawn(async move {
            let sender = sender;
            consumer
                .subscribe(&[topic.as_str()])
                .expect("Can't subscribe to specified topics");
            info!("Start consuming from topic: {}", topic);

            let mut message_stream = consumer.stream();
            while let Some(message) = message_stream.next().await {
                match message {
                    Err(e) => warn!("Kafka strem error: {}", e),
                    Ok(m) => {
                        // let payload = m.payload_view::<str>().unwrap();
                        // let payload = match m.payload_view::<str>() {
                        //     None => "",
                        //     Some(Ok(s)) => s,
                        //     Some(Err(e)) => {
                        //         warn!("Error while deserializing message payload: {:?}", e);
                        //         ""
                        //     }
                        // };
                        // info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}", m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                        // let key = String::from_utf8_lossy(m.key().unwrap()).to_string();
                        let key = if m.key().is_some() {
                            Some(m.key().unwrap().to_vec())
                        } else {
                            None
                        };
                        let payload: Option<Vec<u8>> = if m.payload().is_some() {
                            Some(m.payload().unwrap().to_vec())
                        } else {
                            None
                        };

                        let data = ConsumerMsg {
                            timestamp: m.timestamp().to_millis(),
                            topic: m.topic().to_string(),
                            partition: m.partition(),
                            key: key,
                            payload: payload,
                            offset: m.offset(),
                        };
                        let _ = sender.send(data);
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                    }
                };
            }
        });
        anyhow::Ok(())
    }
}
