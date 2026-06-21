use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_core::basic_cache::BasicCache;
use lru_core::cache::Cache;

fn bench_put(c: &mut Criterion) {
    c.bench_function("cache put", |b| {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        b.iter(|| {
            cache.put(black_box("bench".to_string()), black_box(5)).unwrap();
        });
    });
}
criterion_group!(benches, bench_put);
criterion_main!(benches);