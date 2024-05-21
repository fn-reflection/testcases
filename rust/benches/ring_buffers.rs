use criterion::{black_box, Criterion};
use rb::RbConsumer as _;
use rb::RbProducer as _;
use rb::RB as _;

fn circular_queue(c: &mut Criterion) {
    c.bench_function("ring_buffers::circular_queue::CircularQueue", |b| {
        b.iter(|| {
            let mut q1 = circular_queue::CircularQueue::with_capacity(3);
            for i in 1..=10 {
                q1.push(black_box(i));
            }
            let _ = q1.iter().sum::<i32>();
        })
    });
}

fn spscrb(c: &mut Criterion) {
    c.bench_function("ring_buffers::rb::SpscRb", |b| {
        b.iter(|| {
            let rb = rb::SpscRb::new(3);
            let (tx, rx) = (rb.producer(), rb.consumer());
            let _ = tx.write(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            let receive_arr = &mut [0, 0, 0];
            let _ = rx.read(receive_arr);
            let _ = receive_arr.iter().sum::<i32>();
        })
    });
}

fn arraydeque(c: &mut Criterion) {
    c.bench_function("ring_buffers::arraydeque::ArrayDeque", |b| {
        b.iter(|| {
            let mut q1: arraydeque::ArrayDeque<_, 3> = arraydeque::ArrayDeque::new();
            for i in 1..=10 {
                let _ = q1.push_back(black_box(i));
            }
            let _ = q1.iter().sum::<i32>();
        })
    });
}

fn bounded_vec_deque(c: &mut Criterion) {
    c.bench_function("ring_buffers::bounded_vec_deque::BoundedVecDeque", |b| {
        b.iter(|| {
            let mut q1 = bounded_vec_deque::BoundedVecDeque::new(3);
            for i in 1..=10 {
                let _ = q1.push_back(black_box(i));
            }
            let _ = q1.iter().sum::<i32>();
        })
    });
}

fn lockfree_spsc(c: &mut Criterion) {
    c.bench_function("ring_buffers::lockfree::channel::spsc::create", |b| {
        b.iter(|| {
            let (mut tx, mut rx) = lockfree::channel::spsc::create();
            for i in 1..=10 {
                tx.send(black_box(i)).unwrap();
            }
            let mut _i = 0;
            while let Ok(x) = rx.recv() {
                _i += x;
            }
        })
    });
}

fn benches() {
    let mut criterion = Criterion::default().configure_from_args();
    circular_queue(&mut criterion);
    spscrb(&mut criterion);
    arraydeque(&mut criterion);
    bounded_vec_deque(&mut criterion);
    lockfree_spsc(&mut criterion);
}

fn main() {
    benches();
    ::criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
