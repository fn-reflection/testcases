use futures::stream::StreamExt as _;
const ITERATIONS: i32 = 1000000;

fn std_channel(c: &mut criterion::Criterion) {
    c.bench_function("std::sync::mpsc::channel", |b| {
        b.iter(|| {
            let (tx, rx) = std::sync::mpsc::channel();
            let t = std::thread::spawn(move || {
                (1..=ITERATIONS).for_each(|n| tx.send(n).unwrap());
                drop(tx);
            });
            let mut _cnt = 0;
            while let Ok(_msg) = rx.recv() {
                _cnt += 1;
            }
            t.join().unwrap();
        })
    });
}

fn bus_bus(c: &mut criterion::Criterion) {
    c.bench_function("bus::Bus", |b| {
        b.iter(|| {
            let mut bus = bus::Bus::new(1024);
            let mut rx1 = bus.add_rx();

            let t = std::thread::spawn(move || {
                (1..=ITERATIONS).for_each(|n| bus.broadcast(n));
                drop(bus);
            });
            let mut _cnt = 0;
            while let Ok(_msg) = rx1.recv() {
                _cnt += 1;
            }
            t.join().unwrap();
        })
    });
}

fn create_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn futures_channel(c: &mut criterion::Criterion) {
    c.bench_function("futures::sync::mpsc::unbounded", |b| {
        b.iter(|| {
            create_tokio_rt().spawn(async {
                let (tx, rx) = futures::channel::mpsc::unbounded();
                let fut_send = async move {
                    (1..=ITERATIONS).for_each(|n| tx.unbounded_send(n).unwrap());
                    drop(tx);
                };
                let mut _cnt = 0;
                let fut_rcv = rx.for_each(|_| {
                    _cnt += 1;
                    std::future::ready(())
                });
                tokio::join!(fut_send, fut_rcv);
            });
        })
    });
}

fn tokio_channel(c: &mut criterion::Criterion) {
    c.bench_function("tokio::sync::mpsc::unbounded_channel", |b| {
        b.iter(|| {
            create_tokio_rt().block_on(async {
                let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
                let fut_send = async move {
                    for n in 1..=ITERATIONS {
                        tx.send(n).await.unwrap();
                    }
                };
                let fut_rcv = async move {
                    let mut _cnt = 0;
                    while let Some(_x) = rx.recv().await {
                        _cnt += 1;
                    }
                };
                tokio::join!(fut_send, fut_rcv);
            });
        })
    });
}

fn bondi(c: &mut criterion::Criterion) {
    c.bench_function("bondi::Bondi", |b| {
        b.iter(|| {
            let bondi = bondi::Bondi::new(100);
            let tx = bondi.get_tx().unwrap();
            let rx = bondi.get_rx().unwrap();
            let t = std::thread::spawn(move || {
                (1..=ITERATIONS).for_each(|n| tx.write(Some(n)));
                tx.write(None);
            });
            let mut _cnt = 0;
            while let Some(_msg) = rx.read() {
                _cnt += 1;
            }
            t.join().unwrap();
        })
    });
}

fn eventador(c: &mut criterion::Criterion) {
    c.bench_function("eventador::Eventador", |b| {
        b.iter(|| {
            let eventbus = eventador::Eventador::new(1024).unwrap();
            let subscriber = eventbus.subscribe::<Option<i32>>();
            let t = std::thread::spawn(move || {
                (1..=ITERATIONS).for_each(|n| eventbus.publish(Some(n)));
                eventbus.publish::<Option<i32>>(None);
            });
            let mut _cnt = 0;
            while let Some(_msg) = *(subscriber.recv()) {
                _cnt += 1;
            }
            t.join().unwrap();
        })
    });
}

criterion::criterion_group!(
    benches,
    std_channel,
    bus_bus,
    futures_channel,
    tokio_channel,
    bondi,
    eventador,
);
criterion::criterion_main!(benches);
