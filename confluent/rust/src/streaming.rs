use env_logger::fmt::Formatter;
use futures::TryStreamExt;
use log::info;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use std::io::Write as _;
use std::thread;
use std::time::Duration;

fn create_stream_consumer(brokers: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "example_consumer_group_id")
        .set("enable.partition.eof", "false")
        .set("enable.auto.commit", "false")
        .set("session.timeout.ms", "6000")
        .create::<StreamConsumer>()
        .unwrap()
}

fn create_future_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .unwrap()
}

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
    let brokers = "localhost:9092";
    let consumer = create_stream_consumer(brokers);
    consumer.subscribe(&["input-topic"]).unwrap();

    let producer = create_future_producer(brokers);
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        async move {
            tokio::spawn(streaming(producer.clone(), borrowed_message.detach()));
            Ok(())
        }
    });
    info!("Starting event loop");
    stream_processor.await.unwrap();
}

fn main() {
    env_logger::Builder::new()
        .format(move |formatter: &mut Formatter, record: &log::Record| {
            let thread_name = format!("(t: {}) ", thread::current().name().unwrap());
            write!(
                formatter,
                "{}{} - {} - {}\n",
                thread_name,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .parse_filters("rdkafka=trace")
        .init();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(run());
}
