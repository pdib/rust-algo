extern crate test;

use test::Bencher;
use super::unefficient_queue::Queue;
use super::linked_list_fifth::List;

#[bench]
fn bench_unefficient_dequeue(b: &mut Bencher) {
    b.iter(|| {
        let mut q = Queue::new();
        for i in 0..1000 {
            q.enqueue(i);
        }
        for _ in 0..1000 {
            q.dequeue();
        }
    })
}

#[bench]
fn bench_efficient_dequeue(b: &mut Bencher) {
    b.iter(|| {
        let mut l = List::new();
        for i in 0..1000 {
            l.push(i);
        }
        for _ in 0..1000 {
            l.pop();
        }
    })
}


