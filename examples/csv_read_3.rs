// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// sample for explain read from file system

// Date,Open,High,Low,Close,Volume
use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: f32,
    #[serde(rename = "High")]
    high: f32,
    #[serde(rename = "Low")]
    low:f32,
    #[serde(rename = "Close")]
    close:f32,
    #[serde(rename = "Volume")]
    volume:f32,
}

fn main() -> Result<(), csv::Error> {
//     let _csv = "year,make,model,description
// 1948,Porsche,356,Luxury sports car
// 1967,Ford,Mustang fastback 1967,American car";


    // let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();



    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "{},{},{},{},{},{}",
            record.date,
            record.open,
            record.high,
            record.low,
            record.close,
            record.volume,
        );
    }

    Ok(())
}

//cargo run --example csv_read_1