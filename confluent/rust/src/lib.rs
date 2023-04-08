use env_logger::fmt::Formatter;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use std::io::Write as _;
use std::thread;

pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/protobuf.v1.rs"));
}

pub fn create_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

pub fn create_future_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .unwrap()
}

pub fn create_stream_consumer(brokers: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "example_consumer_group_id")
        .set("enable.partition.eof", "false")
        .set("enable.auto.commit", "false")
        .set("session.timeout.ms", "6000")
        .create::<StreamConsumer>()
        .unwrap()
}

pub fn init_logger() {
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
}
