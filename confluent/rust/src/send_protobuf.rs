use futures::TryStreamExt as _;
use prost::Message as _;
use rand::Rng as _;
use rdkafka::{consumer::Consumer as _, producer::FutureRecord, Message as _};
use rust_studies_kafka::{
    create_future_producer, create_stream_consumer, create_tokio_rt, init_logger, protobuf::Metrics,
};
use std::time::Duration;

async fn producer(brokers: &str) {
    let producer = create_future_producer(brokers);
    loop {
        let record = Metrics {
            time: chrono::Utc::now().to_rfc3339(),
            unit: "ms".to_string(),
            http_method: "GET".to_string(),
            value: rand::thread_rng().gen_range(1..5),
            http_code: "200".to_string(),
            page: "index".to_string(),
            metric_type: "counter".to_string(),
            server: "localhost".to_string(),
        }
        .encode_to_vec();
        let record = FutureRecord::to("metrics_pb")
            .key("some key")
            .payload(&record);
        let res = producer.send(record, Duration::from_secs(0)).await.unwrap();
        dbg!(res);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn consumer(brokers: &str) {
    let consumer = create_stream_consumer(brokers);
    consumer.subscribe(&["metrics_pb"]).unwrap();
    consumer
        .stream()
        .try_for_each(|borrow_message| async move {
            let record = Metrics::decode(borrow_message.payload().unwrap()).unwrap();
            dbg!(record);
            Ok(())
        })
        .await
        .unwrap();
}

async fn run() {
    let brokers = "localhost:9092";
    producer(brokers).await;
    // consumer(brokers).await;
}

fn main() {
    init_logger();
    create_tokio_rt().block_on(run());
}
