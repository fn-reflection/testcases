use futures::TryStreamExt;
use log::info;
use rdkafka::consumer::Consumer;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use rust_studies_kafka::{
    create_future_producer, create_stream_consumer, create_tokio_rt, init_logger,
};
use std::time::Duration;

// derives new message from consumed message.
// be called with spawn_blocking in async context
// because this will not be over soon.
fn stream_calc(msg: OwnedMessage) -> String {
    info!(
        "stream_calc received message {:?}",
        msg.payload_view::<str>()
    );
    match msg.payload_view::<str>() {
        Some(Ok(payload)) => payload,
        Some(Err(_)) => "fail_to_deserialize",
        None => "no_payload",
    }
    .to_string()
}

async fn streaming(producer: FutureProducer, owned_message: OwnedMessage) {
    let calculated = tokio::task::spawn_blocking(|| stream_calc(owned_message))
        .await
        .unwrap();
    let record = FutureRecord::to("output-topic")
        .key("some key")
        .payload(&calculated);
    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => println!("Error: {:?}", e),
    }
}

async fn run() {
    let brokers = "localhost:39092";
    let consumer = create_stream_consumer(brokers);
    consumer.subscribe(&["input-topic"]).unwrap();

    let producer = create_future_producer(brokers);
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        async move {
            tokio::spawn(streaming(producer, borrowed_message.detach()));
            Ok(())
        }
    });
    info!("Starting event loop");
    stream_processor.await.unwrap();
}

fn main() {
    init_logger();
    create_tokio_rt().block_on(run());
}
