use criterion::black_box;
use rb::RbConsumer as _;
use rb::RbProducer as _;
use rb::RB as _;

fn circular_queue(c: &mut criterion::Criterion) {
    c.bench_function("ring_buffers::circular_queue::CircularQueue", |b| {
        b.iter(|| {
            let mut q1 = circular_queue::CircularQueue::with_capacity(3);
            for i in 1..=10 {
                q1.push(black_box(i));
            }
            let _ = q1.iter().fold(0, |acc, x| acc + x);
        })
    });
}

fn spscrb(c: &mut criterion::Criterion) {
    c.bench_function("ring_buffers::rb::SpscRb", |b| {
        b.iter(|| {
            let rb = rb::SpscRb::new(3);
            let (tx, rx) = (rb.producer(), rb.consumer());
            let _ = tx.write(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            let receive_arr = &mut [0, 0, 0];
            let _ = rx.read(receive_arr);
            let _ = receive_arr.iter().fold(0, |acc, x| acc + x);
        })
    });
}

fn arraydeque(c: &mut criterion::Criterion) {
    c.bench_function("ring_buffers::arraydeque::ArrayDeque", |b| {
        b.iter(|| {
            let mut q1: arraydeque::ArrayDeque<[_; 3]> = arraydeque::ArrayDeque::new();
            for i in 1..=10 {
                let _ = q1.push_back(black_box(i));
            }
            let _ = q1.iter().fold(0, |acc, v| acc + v);
        })
    });
}

fn bounded_vec_deque(c: &mut criterion::Criterion) {
    c.bench_function("ring_buffers::bounded_vec_deque::BoundedVecDeque", |b| {
        b.iter(|| {
            let mut q1 = bounded_vec_deque::BoundedVecDeque::new(3);
            for i in 1..=10 {
                let _ = q1.push_back(black_box(i));
            }
            let _ = q1.iter().fold(0, |acc, v| acc + v);
        })
    });
}

criterion::criterion_group!(benches, circular_queue, spscrb, arraydeque, bounded_vec_deque);
criterion::criterion_main!(benches);
