use criterion::{black_box, criterion_group, criterion_main, Criterion};
use loro::{LoroDoc, LoroMap, LoroText};
use std::time::Instant;

fn bench_loro(c: &mut Criterion) {
    let mut group = c.benchmark_group("Loro Benchmarks");
    bench_version_016(
        &mut group,
        "0 Old Snapshot Format on 0.16.12",
        include_bytes!("../dataset/v016snapshot.bin"),
        |doc: &loro_016::LoroDoc| {
            #[allow(deprecated)]
            doc.export_snapshot()
        },
    );

    bench_version(
        &mut group,
        "1 Old Snapshot Format on 1.0.0",
        include_bytes!("../dataset/v016snapshot.bin"),
        |doc: &LoroDoc| {
            #[allow(deprecated)]
            doc.export_snapshot()
        },
    );

    bench_version(
        &mut group,
        "2 New Snapshot Format",
        include_bytes!("../dataset/fast_snapshot.bin"),
        |doc: &LoroDoc| doc.export(loro::ExportMode::Snapshot).unwrap(),
    );

    bench_version(
        &mut group,
        "3 Shallow Snapshot Format",
        include_bytes!("../dataset/shallow_snapshot.bin"),
        |doc: &LoroDoc| doc.export(loro::ExportMode::Snapshot).unwrap(),
    );

    group.finish();
}

fn bench_version<F>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    input: &'static [u8],
    export_fn: F,
) where
    F: Fn(&LoroDoc) -> Vec<u8> + 'static,
{
    group.bench_function(format!("{} - Import", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                black_box(doc);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                black_box(doc);
                black_box(value);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues+Edit", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                let list = doc.get_list("cells");
                let cell = list.push_container(LoroMap::new()).unwrap();
                cell.insert("cellType", "markdown").unwrap();
                let text = cell.insert_container("source", LoroText::new()).unwrap();
                text.insert(0, "Hello world!").unwrap();
                doc.commit();
                black_box(doc);
                black_box(value);
                black_box(text);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues+Edit+Export", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                let list = doc.get_list("cells");
                let cell = list.push_container(LoroMap::new()).unwrap();
                cell.insert("cellType", "markdown").unwrap();
                let text = cell.insert_container("source", LoroText::new()).unwrap();
                text.insert(0, "Hello world!").unwrap();
                doc.commit();
                let snapshot = export_fn(&doc);
                black_box(snapshot);
                black_box(doc);
                black_box(value);
                black_box(text);
            }
            start.elapsed()
        });
    });
}

fn bench_version_016<F>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &str,
    input: &'static [u8],
    export_fn: F,
) where
    F: Fn(&loro_016::LoroDoc) -> Vec<u8> + 'static,
{
    group.bench_function(format!("{} - Import", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = loro_016::LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                black_box(doc);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = loro_016::LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                black_box(doc);
                black_box(value);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues+Edit", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = loro_016::LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                let list = doc.get_list("cells");
                let cell = list.push_container(loro_016::LoroMap::new()).unwrap();
                cell.insert("cellType", "markdown").unwrap();
                let text = cell
                    .insert_container("source", loro_016::LoroText::new())
                    .unwrap();
                text.insert(0, "Hello world!").unwrap();
                doc.commit();
                black_box(doc);
                black_box(value);
                black_box(text);
            }
            start.elapsed()
        });
    });

    group.bench_function(format!("{} - Import+GetAllValues+Edit+Export", name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let doc = loro_016::LoroDoc::new();
                doc.import(black_box(input)).unwrap();
                let value = doc.get_deep_value();
                let list = doc.get_list("cells");
                let cell = list.push_container(loro_016::LoroMap::new()).unwrap();
                cell.insert("cellType", "markdown").unwrap();
                let text = cell
                    .insert_container("source", loro_016::LoroText::new())
                    .unwrap();
                text.insert(0, "Hello world!").unwrap();
                doc.commit();
                let snapshot = export_fn(&doc);
                black_box(snapshot);
                black_box(doc);
                black_box(value);
                black_box(text);
            }
            start.elapsed()
        });
    });
}

criterion_group!(benches, bench_loro);
criterion_main!(benches);
