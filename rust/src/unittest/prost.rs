use crate::board::Board as ProtoBoard;
use chrono::NaiveDateTime;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Board {
    pub id: Option<i64>,
    pub time: chrono::NaiveDateTime,
    pub asks_d: BTreeMap<OrderedFloat<f64>, f32>,
    pub bids_d: BTreeMap<OrderedFloat<f64>, f32>,
    pub q_id: i32,
    pub spread: Option<f64>,
}

impl From<ProtoBoard> for Board {
    fn from(pb: ProtoBoard) -> Self {
        let time = pb.time.unwrap();
        let time = NaiveDateTime::from_timestamp_opt(time.seconds, time.nanos as u32).unwrap();

        let asks_d = pb
            .asks_d
            .into_iter()
            .map(|order| (OrderedFloat(order.price), order.volume))
            .collect();

        let bids_d = pb
            .bids_d
            .into_iter()
            .map(|order| (OrderedFloat(order.price), order.volume))
            .collect();

        Board {
            id: if pb.id == 0 { None } else { Some(pb.id) },
            time,
            asks_d,
            bids_d,
            q_id: pb.q_id,
            spread: if pb.spread == 0.0 {
                None
            } else {
                Some(pb.spread)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use crate::board::{Board as ProtoBoard, Order as ProtoOrder};
    use crate::test_proto::{Obj as ProtoObj, Status as ProtoStatus, TestProto as ProtoTestProto};
    use prost::Message as _;

    #[test]
    fn from_prost() {
        let prost_board = ProtoBoard {
            id: 1,
            time: Some(prost_types::Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            asks_d: vec![ProtoOrder {
                price: 100.0,
                volume: 1.0,
            }],
            bids_d: vec![],
            q_id: 1,
            spread: 0.0,
        };
        let board = Board::from(prost_board);
        dbg!(board);
    }

    #[test]
    fn test_proto() {
        let prost_test = ProtoTestProto {
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
        let decoded = ProtoTestProto::decode(encoded.as_ref()).unwrap();
        dbg!(decoded);
    }
}
