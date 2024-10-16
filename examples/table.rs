use serde::{Deserialize, Serialize};
use serde_columnar::Itertools;
use std::{fs::File, path::PathBuf, time::Duration};
use tabled::settings::Style;

pub fn main() {
    use std::fs;
    use tabled::Table;

    let mut rows = Vec::new();
    let criterion_dir = "./target/criterion/Loro Benchmarks/";
    let mut targets = vec![];
    if let Ok(entries) = fs::read_dir(criterion_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let path = entry.path();
                    if let Some(path_str) = path.to_str() {
                        targets.push(path_str.to_string());
                    }
                }
            }
        }
    } else {
        println!("Failed to read criterion directory. You should call `cargo bench` before using this program.");
        return;
    }

    targets.sort();
    for target in targets {
        let row = gen_benchmark_row(&target);
        rows.push(row);
    }

    let table = Table::new(rows.clone())
        .with(tabled::settings::merge::Merge::vertical())
        .to_string();
    println!("{}", table);
    println!();
    println!();
    println!();
    println!();

    let md_table = Table::new(rows)
        .with(tabled::settings::merge::Merge::vertical())
        .with(Style::markdown())
        .to_string();
    println!("{}", md_table);
}

#[derive(tabled::Tabled, Clone)]
struct TableRow {
    name: String,
    task: String,
    time: String,
}

fn gen_benchmark_row(path_to_criterion_result: &str) -> TableRow {
    let mut path = PathBuf::from(path_to_criterion_result);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let split = file_name.split(" - ").collect_vec();
    let name = split[0].to_string();
    let task = split[1].to_string();
    path.push("new");
    path.push("estimates.json");
    println!("path: {}", path.to_str().unwrap());
    let file = File::open(path).unwrap();
    let parsed: BenchmarkEstimates = serde_json::from_reader(file).unwrap();
    TableRow {
        name,
        task,
        time: format!(
            "{:?} +- {:?}",
            Duration::from_nanos(parsed.mean.point_estimate as u64),
            Duration::from_nanos(parsed.mean.standard_error as u64)
        ),
    }
}

#[derive(Serialize, Deserialize)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Serialize, Deserialize)]
struct Estimate {
    confidence_interval: ConfidenceInterval,
    point_estimate: f64,
    standard_error: f64,
}

#[derive(Serialize, Deserialize)]
struct BenchmarkEstimates {
    mean: Estimate,
    median: Estimate,
    median_abs_dev: Estimate,
    #[serde(default)]
    slope: Option<Estimate>,
    std_dev: Estimate,
}
