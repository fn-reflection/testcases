use crate::protobuf::v1::Migrate as ProtoV1Migrate;
use chrono::DateTime;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct MigrateV1 {
    pub i: Option<i64>,
    pub t: chrono::DateTime<chrono::Utc>,
    pub bc1: BTreeMap<OrderedFloat<f64>, f32>,
    pub bc2: BTreeMap<OrderedFloat<f64>, f32>,
    pub d: Option<f64>,
}

impl From<ProtoV1Migrate> for MigrateV1 {
    fn from(pb: ProtoV1Migrate) -> Self {
        let t = pb.t.unwrap();
        let t = DateTime::from_timestamp(t.seconds, t.nanos as u32).unwrap();

        let bc1 = pb
            .bc1
            .into_iter()
            .map(|bc| (OrderedFloat(bc.d), bc.f))
            .collect();

        let bc2 = pb
            .bc2
            .into_iter()
            .map(|bc| (OrderedFloat(bc.d), bc.f))
            .collect();

        MigrateV1 {
            i: if pb.i == 0 { None } else { Some(pb.i) },
            t,
            bc1,
            bc2,
            d: if pb.d == 0.0 { None } else { Some(pb.d) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MigrateV1;
    use crate::protobuf::v1::{
        Exhaustive as ProtoExhaustive, Migrate as ProtoV1Migrate,
        MigrateChild as ProtoV1MigrateChild, Obj as ProtoObj, Status as ProtoStatus,
    };
    use crate::protobuf::v2::Migrate as ProtoV2Migrate;
    use prost::Message as _;

    #[test]
    fn struct_from_protobuf_data_ok() {
        let prost_v1_migrate = ProtoV1Migrate {
            i: 1,
            t: Some(prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            bc1: vec![ProtoV1MigrateChild { d: 100.0, f: 1.0 }],
            bc2: vec![],
            d: 0.0,
        };
        let board = MigrateV1::from(prost_v1_migrate);
        dbg!(board);
    }

    #[test]
    fn serde_exhaustive_data_ok() {
        let prost_test = ProtoExhaustive {
            int: 1,
            int_opt: Some(1),
            ints: vec![1],
            fl: 1.0,
            fl_opt: Some(1.0),
            fls: vec![1.0],
            flag: true,
            flag_opt: Some(true),
            flags: vec![true],
            str: "a".to_string(),
            str_opt: Some("a".to_string()),
            strs: vec!["a".to_string()],
            buf: vec![1, 2, 3],
            buf_opt: Some(vec![1, 2, 3]),
            bufs: vec![vec![1, 2, 3]],
            status: ProtoStatus::Active as i32,
            status_opt: Some(ProtoStatus::Active as i32),
            statuses: vec![ProtoStatus::Active as i32],
            map: maplit::hashmap! {1 => "abc".to_string()},
            obj: Some(ProtoObj {
                int: 1,
                opt_int: Some(2),
            }),
            obj_opt: Some(ProtoObj {
                int: 1,
                opt_int: Some(2),
            }),
            objs: vec![ProtoObj {
                int: 1,
                opt_int: Some(2),
            }],
            time: Some(prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            time_opt: Some(prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            times: vec![prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }],
            dur: Some(prost_types::Duration {
                seconds: 1,
                nanos: 1,
            }),
            dur_opt: Some(prost_types::Duration {
                seconds: 1,
                nanos: 1,
            }),
            durs: vec![prost_types::Duration {
                seconds: 1,
                nanos: 1,
            }],
        };
        let mut buf = vec![];
        prost_test.encode(&mut buf).unwrap();
        dbg!(buf);
        dbg!(prost_test.encoded_len());
        let encoded = prost_test.encode_to_vec();
        dbg!(prost_test);
        let decoded = ProtoExhaustive::decode(encoded.as_ref()).unwrap();
        dbg!(decoded);
    }

    #[test]
    fn generate_v2_from_v1_ok() {
        let prost_v1_migrate = ProtoV1Migrate {
            i: 1,
            t: Some(prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            bc1: vec![ProtoV1MigrateChild { d: 100.0, f: 1.0 }],
            bc2: vec![],
            d: 0.0,
        };
        let encoded = prost_v1_migrate.encode_to_vec();
        dbg!(prost_v1_migrate);
        let decoded = ProtoV2Migrate::decode(encoded.as_ref()).unwrap();
        assert_eq!(decoded.bc1[0].d2, 0.0); // default
        assert_eq!(decoded.bc1[0].dopt, None); // default
        assert_eq!(decoded.s, "");
        dbg!(decoded);
    }
}
