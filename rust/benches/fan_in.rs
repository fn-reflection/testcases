use chrono::NaiveDateTime;
use std::sync::{atomic::AtomicI64, Arc, Mutex};

const ITERATIONS: i32 = 1000000;
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
struct Measurement {
    time: NaiveDateTime,
    value: f64,
    thread_id: usize,
}

impl Default for Measurement {
    fn default() -> Self {
        Measurement {
            time: chrono::Utc::now().naive_utc(),
            value: std::f64::NAN,
            thread_id: 0,
        }
    }
}

impl Measurement {
    fn new(value: f64, thread_id: usize) -> Self {
        Measurement {
            time: chrono::Utc::now().naive_utc(),
            value,
            thread_id,
        }
    }
}

fn create_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn std_mutex_vecdeque(c: &mut criterion::Criterion) {
    c.bench_function("fan_in::Mutex<VecDeque<Measurement>>", |b| {
        b.iter(|| {
            let queue = std::collections::VecDeque::new();
            let pair = Arc::new((Mutex::new(queue), AtomicI64::new(0)));
            let pair1 = pair.clone();
            let record_thread1 = std::thread::spawn(move || {
                for i in 1..=ITERATIONS {
                    let m = Measurement::new(i as f64, 1);
                    pair1.0.lock().unwrap().push_back(m);
                }
                pair1.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            let pair2 = pair.clone();
            let record_thread2 = std::thread::spawn(move || {
                for i in 1..=ITERATIONS {
                    let m = Measurement::new(i as f64, 2);
                    pair2.0.lock().unwrap().push_back(m);
                }
                pair2.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            let consume_thread = {
                std::thread::spawn(move || {
                    let mut result = Vec::with_capacity(2000000);
                    while !(pair.1.load(std::sync::atomic::Ordering::SeqCst) == 2
                        && pair.0.lock().unwrap().len() == 0)
                    {
                        let mut queue_lock = pair.0.lock().unwrap();
                        queue_lock
                            .iter_mut()
                            .for_each(|m| result.push(std::mem::take(m)));
                        queue_lock.clear();
                    }
                })
            };
            for thread in [record_thread1, record_thread2, consume_thread] {
                let _ = thread.join();
            }
        })
    });
}

fn std_channel(c: &mut criterion::Criterion) {
    c.bench_function("fan_in::std::sync::mpsc::channel", |b| {
        b.iter(|| {
            let (tx1, rx) = std::sync::mpsc::channel();
            let tx2 = tx1.clone();
            let record_thread1 = std::thread::spawn(move || {
                for i in 1..=ITERATIONS {
                    let m = Measurement::new(i as f64, 1);
                    let _ = tx1.send(m);
                }
            });
            let record_thread2 = std::thread::spawn(move || {
                for i in 1..=ITERATIONS {
                    let m = Measurement::new(i as f64, 2);
                    let _ = tx2.send(m);
                }
            });
            let mut result = Vec::with_capacity(2000000);
            let consume_thread = {
                std::thread::spawn(move || {
                    rx.iter().for_each(|m| result.push(m));
                })
            };
            for thread in [record_thread1, record_thread2, consume_thread] {
                let _ = thread.join();
            }
        })
    });
}

fn tokio_channel(c: &mut criterion::Criterion) {
    c.bench_function("fan_in::tokio::sync::mpsc::unbounded_channel", |b| {
        b.iter(|| {
            create_tokio_rt().block_on(async {
                let (tx1, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let tx2 = tx1.clone();
                let fut_send1 = async move {
                    for i in 1..=ITERATIONS {
                        let m = Measurement::new(i as f64, 1);
                        tx1.send(m).unwrap();
                    }
                };
                let fut_send2 = async move {
                    for i in 1..=ITERATIONS {
                        let m = Measurement::new(i as f64, 2);
                        tx2.send(m).unwrap();
                    }
                };
                let fut_rcv = async move {
                    let mut result = Vec::with_capacity(2000000);
                    while let Some(m) = rx.recv().await {
                        result.push(m);
                    }
                };
                tokio::join!(fut_send1, fut_send2, fut_rcv);
            });
        })
    });
}

criterion::criterion_group!(benches, std_mutex_vecdeque, std_channel, tokio_channel);
criterion::criterion_main!(benches);
