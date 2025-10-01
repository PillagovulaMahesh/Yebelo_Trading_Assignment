use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn create_consumer(
    brokers: &str,
    group_id: &str,
    topic: &str,
) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic]).expect("Can't subscribe to topic");
    consumer
}

pub async fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Producer creation error")
}

pub async fn consume_messages<F>(consumer: StreamConsumer, mut callback: F)
where
    F: FnMut(rdkafka::message::BorrowedMessage) + Send + 'static + Copy,
{
    let mut stream = consumer.stream();
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => callback(msg),
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}

pub fn publish_rsi(producer: &FutureProducer, topic: &str, msg: String) -> bool {
    let record = FutureRecord::to(topic).payload(&msg).key("");
    let _ = producer.send(record, Timeout::After(Duration::from_secs(0)));
    true
}
