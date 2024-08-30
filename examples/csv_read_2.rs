// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// MOTIVATION read inside a fn

// sample for explain read from file system

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rust_decimal::prelude::FromPrimitive;
// use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

//use chrono::DateTime;
//use chrono::NaiveDateTime;
//use chrono::Utc;

// Date,Open,High,Low,Close,Volume
use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "Volume")]
    volume: f64,
}

#[derive(Debug)]
pub struct StockData {
    date: DateTime<Utc>,
    high: Decimal,
    low: Decimal,
    open: Decimal,
    close: Decimal,
    #[allow(dead_code)]
    net_change: Option<Decimal>,
    #[allow(dead_code)]
    net_change_percent: Option<Decimal>,
}

impl StockData {
    pub fn new(
        date: DateTime<Utc>,
        high: Decimal,
        low: Decimal,
        open: Decimal,
        close: Decimal,
    ) -> Self {
        Self {
            date,
            high,
            low,
            open,
            close,
            net_change: None,
            net_change_percent: None,
        }
    }
}

fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
    let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
    Utc.from_utc_datetime(&day_one)
}

fn read_csv_to_vector(file_name: &str) -> Vec<StockData> {


    let mut  return_vec: Vec<StockData>;
    // let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut reader = csv::Reader::from_path(file_name).unwrap();

    for record in reader.deserialize() {
        
        
        
        println!(
            "{},{},{},{},{},{}",
            record.date, record.open, record.high, record.low, record.close, record.volume,
        );

        //
        // let base_data_series_len = reader.len();

        // let mut rng = rand::thread_rng();

        let high = Decimal::from_f64(record.high).unwrap().round_dp(2);
        let low = Decimal::from_f64(record.low).unwrap().round_dp(2);
        let open = Decimal::from_f64(record.open).unwrap().round_dp(2);
        let close = Decimal::from_f64(record.close).unwrap().round_dp(2);
        //

        println!("{},{},{},{},{}", record.date, open, high, low, close);

        let line_stock_data =StockData::new(
            generate_utc_date_from_date_string(&record.date),
            high,
            low,
            open,
            close,
        );

        return_vec.push(line_stock_data);
        
    }

    // let return_vec: StockData;

    return_vec
}

fn main() -> Result<(), csv::Error> {
    let file_name = "stock_data/stock_trex_data.csv";

    // let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "{},{},{},{},{},{}",
            record.date, record.open, record.high, record.low, record.close, record.volume,
        );
    }

    Ok(())
}

//cargo run --example
