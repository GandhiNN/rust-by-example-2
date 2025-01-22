use criterion::{Criterion, criterion_group, criterion_main};

fn bench_unbuffered_one_character_at_a_time(c: &mut Criterion) {
    c.bench_function("unbuffered_one_character_at_a_time", |b| {
        b.iter(|| read_unbuffered_one_character_at_a_time())
    });
}
