// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// sample for explain read from file system

use csv::Error;
use chrono::{DateTime,/*NaiveDate,*/NaiveDateTime, TimeZone, Utc};
// use rust_decimal::prelude::FromPrimitive;
// use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

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
    low: f32,
    #[serde(rename = "Close")]
    close: f32,
    #[serde(rename = "Volume")]
    #[allow(dead_code)]
    volume: f32,
}

#[derive(Debug)]
pub struct StockData {
    #[allow(dead_code)]
    date: DateTime<Utc>,
    #[allow(dead_code)]
    high: Decimal,
    #[allow(dead_code)]
    low: Decimal,
    #[allow(dead_code)]
    open: Decimal,
    #[allow(dead_code)]
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

    // poor debug
   //  println!("date-string => {}",  date_string);
   
    let date_time_string = format!("{} 00:00:00",date_string);
    let day_one = NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %H:%M:%S").unwrap();
    
    Utc.from_utc_datetime(&day_one)
        
}

fn main() {
    println!("Start");

    let stock_data= read_csv();

    // println!("Len => {}",stock_data.unwrap().len());

    vec_loop_3(stock_data);

}

fn vec_loop_3(mut v: Result<Vec<StockData>,Error>) {
    for i in v.iter_mut() {
        println!("{:?}",i);
        println!("<>");
    }
}

fn read_csv() -> Result<Vec<StockData>, csv::Error> {
    // let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    // https://www.geeksforgeeks.org/rust-vectors/
    let mut stock_data:Vec<StockData> = Vec::new(); 
    let mut stk_line;
    // let fmt = "%Y-%m-%d";

    for record in reader.deserialize() {
        let record: Record = record?;
        // println!(
        //     "{},{},{},{},{},{}",
        //     record.date, record.open, record.high, record.low, record.close, record.volume,
        // );

        //let date2= DateTime::parse_from_str(&record.date ,fmt)
        //.unwrap();

    // let date = DateTime::parse_from_str(&record.date, "%Y-%m-%d").unwrap();
    // https://docs.rs/dateparser/latest/dateparser/
    let date = generate_utc_date_from_date_string(&record.date);

    //ohlcv
        let open = Decimal::from_f32_retain(record.open).unwrap()
        .round_dp(2);
        let high = Decimal::from_f32_retain(record.high).unwrap()
        .round_dp(2);
        let low = Decimal::from_f32_retain(record.low).unwrap()
        .round_dp(2);
        let close = Decimal::from_f32_retain(record.close).unwrap()
        .round_dp(2);

        // println!("stk_line => {},{},{},{},{}",date,high,low,open,close);
        stk_line = StockData::new(date,high,low,open,close);

        stock_data.push(stk_line);
    }

    Ok(stock_data)
}

//cargo run --example csv_read_3
