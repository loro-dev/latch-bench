use criterion::black_box;
use dev_utils::{get_mem_usage, ByteSize};
use loro::{LoroDoc, LoroMap, LoroText};
use std::time::Instant;

fn main() {
    bench_016(
        "Old Snapshot Format on 0.16.12",
        include_bytes!("../dataset/v016snapshot.bin"),
        |doc| {
            #[allow(deprecated)]
            doc.export_snapshot()
        },
    );
    bench(
        "Old Snapshot Format on 1.0.0",
        include_bytes!("../dataset/v016snapshot.bin"),
        |doc| {
            #[allow(deprecated)]
            doc.export_snapshot()
        },
    );
    bench(
        "New Snapshot Format",
        include_bytes!("../dataset/fast_snapshot.bin"),
        |doc| doc.export(loro::ExportMode::Snapshot).unwrap(),
    );
    bench(
        "Shallow Snapshot Format",
        include_bytes!("../dataset/shallow_snapshot.bin"),
        |doc| doc.export(loro::ExportMode::Snapshot).unwrap(),
    );
}

fn bench(name: &str, input: &[u8], f: impl Fn(&LoroDoc) -> Vec<u8>) {
    println!("============================");
    println!("    {}", name);
    println!("============================");
    println!("Snapshot Size: {}", ByteSize(input.len()));

    let mem_start = get_mem_usage();
    let start = Instant::now();
    let doc = LoroDoc::new();
    doc.import(input).unwrap();
    println!("Parse Time: {:?}", start.elapsed());
    println!("Memory Usage {:?}", get_mem_usage() - mem_start);
    println!("Ops Size {}", doc.len_ops());
    let _value = doc.get_deep_value();
    println!("Parse + get_deep_value() time: {:?}", start.elapsed());

    let list = doc.get_list("cells");
    let cell = list.push_container(LoroMap::new()).unwrap();
    cell.insert("cellType", "markdown").unwrap();
    let text = cell.insert_container("source", LoroText::new()).unwrap();
    text.insert(0, "Hello world!").unwrap();
    doc.commit();
    println!(
        "Parse + get_deep_value() + PushNewCell time: {:?}",
        start.elapsed()
    );

    let snapshot = f(&doc);
    black_box(snapshot);
    println!(
        "Parse + get_deep_value() + PushNewCell + Export time: {:?}",
        start.elapsed()
    );
    println!(
        "Memory Usage After All Operations {:?}",
        get_mem_usage() - mem_start
    );
    println!("\n\n");
}

fn bench_016(name: &str, input: &[u8], f: impl Fn(&loro_016::LoroDoc) -> Vec<u8>) {
    println!("============================");
    println!("    {}", name);
    println!("============================");
    println!("Snapshot Size: {}", ByteSize(input.len()));

    let mem_start = get_mem_usage();
    let start = Instant::now();
    let doc = loro_016::LoroDoc::new();
    doc.import(input).unwrap();
    println!("Parse Time: {:?}", start.elapsed());
    println!("Memory Usage {:?}", get_mem_usage() - mem_start);
    doc.get_deep_value();
    println!("Parse + get_deep_value() time: {:?}", start.elapsed());

    let list = doc.get_list("cells");
    let cell = list.push_container(loro_016::LoroMap::new()).unwrap();
    cell.insert("cellType", "markdown").unwrap();
    let text = cell
        .insert_container("source", loro_016::LoroText::new())
        .unwrap();
    text.insert(0, "Hello world!").unwrap();
    doc.commit();
    println!(
        "Parse + get_deep_value() + PushNewCell time: {:?}",
        start.elapsed()
    );

    let snapshot = f(&doc);
    black_box(snapshot);
    println!(
        "Parse + get_deep_value() + PushNewCell + Export time: {:?}",
        start.elapsed()
    );
    println!(
        "Memory Usage After All Operations {:?}",
        get_mem_usage() - mem_start
    );
    println!("\n\n");
}
