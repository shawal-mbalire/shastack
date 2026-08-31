use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use tempfile::tempdir;

fn bench_sha_new(c: &mut Criterion) {
    c.bench_function("workspace_init", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let path = dir.path().join("bench_ws");
            shastack_cli::workspace::init(
                black_box(path.to_str().unwrap()),
                vec!["Research (LaTeX)"],
            )
            .unwrap();
        })
    });
}

fn bench_sha_add_feature(c: &mut Criterion) {
    c.bench_function("add_feature", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let path = dir.path().join("bench_add");
            shastack_cli::workspace::init(path.to_str().unwrap(), vec![]).unwrap();
            shastack_cli::workspace::add_feature(
                black_box(&path),
                "Research (LaTeX)",
            )
            .unwrap();
        })
    });
}

fn bench_sha_version_bump(c: &mut Criterion) {
    c.bench_function("version_bump", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let path = dir.path().join("bench_ver");
            shastack_cli::workspace::init(path.to_str().unwrap(), vec![]).unwrap();
            let mut version = shastack_cli::workspace::get_version(&path).unwrap();
            version.patch += 1;
            shastack_cli::workspace::set_version(black_box(&path), &version).unwrap();
        })
    });
}

fn bench_event_bus_emit(c: &mut Criterion) {
    c.bench_function("event_bus_emit", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let bus = shastack_cli::events::EventBus::new(dir.path());
            let event = shastack_cli::events::Event::new("bench.test", "bench");
            bus.emit(black_box(event)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_sha_new,
    bench_sha_add_feature,
    bench_sha_version_bump,
    bench_event_bus_emit
);
criterion_main!(benches);
