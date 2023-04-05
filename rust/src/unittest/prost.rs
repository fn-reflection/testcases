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
}
