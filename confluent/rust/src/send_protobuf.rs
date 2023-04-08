use futures::TryStreamExt as _;
use prost::Message as _;
use rand::Rng as _;
use rdkafka::{consumer::Consumer as _, producer::FutureRecord, Message as _};
use rust_studies_kafka::{
    create_future_producer, create_stream_consumer, create_tokio_rt, init_logger,
    protobuf::{Migrate, MigrateChild},
};
use std::time::Duration;

async fn producer(brokers: &str) {
    let producer = create_future_producer(brokers);
    let mut cnt = 0i64;
    loop {
        let record = Migrate {
            i: cnt,
            t: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
            bc1: vec![MigrateChild {
                d: rand::thread_rng().gen_range(100.0..150.0),
                f: rand::thread_rng().gen_range(0.1..5.0),
            }],
            bc2: vec![],
            d: rand::thread_rng().gen_range(0.1..3.0),
        }
        .encode_to_vec();
        let record = FutureRecord::to("protobuf")
            .key("some key")
            .payload(&record);
        let res = producer.send(record, Duration::from_secs(0)).await.unwrap();
        dbg!(res);
        tokio::time::sleep(Duration::from_secs(1)).await;
        cnt += 1;
    }
}

async fn consumer(brokers: &str) {
    let consumer = create_stream_consumer(brokers);
    consumer.subscribe(&["protobuf"]).unwrap();
    consumer
        .stream()
        .try_for_each(|borrow_message| async move {
            let record = Migrate::decode(borrow_message.payload().unwrap()).unwrap();
            dbg!(record);
            Ok(())
        })
        .await
        .unwrap();
}

async fn run() {
    let brokers = "localhost:9092";
    tokio::spawn(producer(brokers));
    consumer(brokers).await;
}

fn main() {
    init_logger();
    create_tokio_rt().block_on(run());
}
