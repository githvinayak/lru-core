use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_core::basic_cache::BasicCache;
use lru_core::cache::Cache;
use lru_core::lru_cache::LruCache;

fn bench_put(c: &mut Criterion) {
    c.bench_function("cache put", |b| {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        b.iter(|| {
            cache.put(black_box("bench".to_string()), black_box(5)).unwrap();
        });
    });
}
fn bench_lru_put(c: &mut Criterion) {
    c.bench_function("lru put", |b| {
        let mut cache: LruCache<String, i32> = LruCache::new(1);
        b.iter(|| {
            cache.put(black_box("bench".to_string()), black_box(5)).unwrap();
        });
    });
}
fn bench_lru_get(c: &mut Criterion) {
    c.bench_function("lru get", |b| {
        let mut cache: LruCache<String, i32> = LruCache::new(1);
            cache.put("bench".to_string(),5).unwrap();
        b.iter(|| {
            cache.get(black_box(&"bench".to_string()));
        });
    });
}
criterion_group!(benches, bench_put,bench_lru_put,bench_lru_get);
criterion_main!(benches);