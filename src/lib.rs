#![allow(deprecated)]
mod latch_data;

pub fn parse() {
    use latch_data::BYTES;
    let doc = loro_old::LoroDoc::new();
    doc.import(BYTES).unwrap();
    let updates = doc.export_json_updates(&Default::default());
    let new_doc = loro::LoroDoc::new();
    let updates_json = serde_json::to_string_pretty(&updates).unwrap();
    std::fs::write("./dataset/updates.json", &updates_json).unwrap();
    new_doc.import_json_updates(updates_json).unwrap();
    let v16_snapshot = new_doc.export_snapshot();
    std::fs::write("./dataset/v016snapshot.bin", v16_snapshot).unwrap();
    let new_snapshot = new_doc.export(loro::ExportMode::Snapshot).unwrap();
    std::fs::write("./dataset/fast_snapshot.bin", new_snapshot).unwrap();
    let new_snapshot = new_doc
        .export(loro::ExportMode::shallow_snapshot_owned(
            new_doc.oplog_frontiers(),
        ))
        .unwrap();
    std::fs::write("./dataset/shallow_snapshot.bin", new_snapshot).unwrap();
    let value = new_doc.get_deep_value();
    let value_json = serde_json::to_string_pretty(&value).unwrap();
    std::fs::write("./dataset/doc.json", value_json).unwrap();
}
