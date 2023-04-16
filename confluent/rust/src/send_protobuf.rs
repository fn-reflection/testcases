use futures::TryStreamExt as _;
use prost::Message as _;
use rand::Rng as _;
use rdkafka::{consumer::Consumer as _, producer::FutureRecord, Message as _};
use rust_studies_kafka::{
    create_future_producer, create_stream_consumer, create_tokio_rt, init_logger,
    protobuf::{to_proto_qualified_name, v1::Metrics},
};
use schema_registry_converter::{
    async_impl::{proto_raw::ProtoRawEncoder, schema_registry::SrSettings},
    schema_registry_common::{self, SchemaType, SubjectNameStrategy, SuppliedSchema},
};
use std::time::Duration;

async fn producer(brokers: &str) {
    let sr_settings = SrSettings::new("http://localhost:8081".to_string());
    let encoder = ProtoRawEncoder::new(sr_settings);
    let producer = create_future_producer(brokers);
    let supplied_schema = Box::new(SuppliedSchema {
        name: Some(to_proto_qualified_name(std::any::type_name::<Metrics>())),
        schema_type: SchemaType::Protobuf,
        schema: include_str!("../../../protos/protobuf/v1/metrics.proto").to_string(),
        references: vec![],
    });
    dbg!(&supplied_schema);
    loop {
        let subject_name_strategy =
            SubjectNameStrategy::RecordNameStrategyWithSchema(supplied_schema.clone());
        let full_name = schema_registry_common::get_subject(&subject_name_strategy).unwrap();
        let record = Metrics {
            time: Some(std::time::SystemTime::now().into()),
            unit: "ms".to_string(),
            http_method: "GET".to_string(),
            value: rand::thread_rng().gen_range(1..5),
            http_code: "200".to_string(),
            page: "index".to_string(),
            metric_type: "counter".to_string(),
            server: "localhost".to_string(),
        }
        .encode_to_vec();
        let payload = encoder
            .encode(record.as_ref(), full_name.as_str(), subject_name_strategy)
            .await
            .unwrap();
        dbg!(&payload);
        let record = FutureRecord::to("metrics_pb")
            .key("some key")
            .payload(&payload);
        let res = producer.send(record, Duration::from_secs(0)).await.unwrap();
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
