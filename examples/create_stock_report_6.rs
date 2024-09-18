// running into folder => /home/user/workspace_rust/rust_fin_stock
// cargo run --example create_stock_report_6

// use core::fmt;
use std::error::Error;
use std::fs;

use chrono::DateTime;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;
use chrono::TimeZone;
use chrono::NaiveDateTime;

// #[allow(unused_imports)]
// use chrono::{ DateTime, /*NaiveDate,*/ NaiveDateTime, TimeZone, Utc };
// use chrono::NaiveDateTime;
// use chrono::TimeZone;
// use chrono::Utc;


use plotters::prelude::BitMapBackend;
use plotters::prelude::CandleStick;
use plotters::prelude::ChartBuilder;
use plotters::prelude::IntoDrawingArea;
use plotters::prelude::LineSeries;
use plotters::prelude::PathElement;
use plotters::prelude::RGBColor;
use plotters::prelude::SeriesLabelPosition;
// use rand::Rng;

use plotters::prelude::full_palette::ORANGE;
use plotters::prelude::full_palette::PURPLE;

use plotters::style::Color;
use plotters::style::IntoFont;
#[allow(unused_imports)]
use plotters::style::BLUE;
use plotters::style::GREEN;
use plotters::style::RED;
use plotters::style::WHITE;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use serde::Deserialize;




//need for read csv
#[derive(Debug)]
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

// FROM HERE
// https://stackoverflow.com/questions/72071616/how-to-get-fmtdisplay-from-a-struct-to-display-it-in-the-fmtdisplay-of-anoth

// impl fmt::Display for StockData {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         println!("{:?}",self.date);

//         Ok(())
//     }
// }

impl StockData {
    pub fn new(
        date: DateTime<Utc>,
        high: Decimal,
        low: Decimal,
        open: Decimal,
        close: Decimal
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
    let date_time_string = format!("{} 00:00:00", date_string);
    let day_one = NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %H:%M:%S").unwrap();

    Utc.from_utc_datetime(&day_one)
}

fn generate_stock_data_series(stock_data: &str) -> Result<Vec<StockData>, csv::Error> {
    // read date from file => stock_data/stock_trex_data.csv
    let mut reader = csv::Reader::from_path(stock_data).unwrap();

    // https://www.geeksforgeeks.org/rust-vectors/
    let mut stock_data: Vec<StockData> = Vec::new();
    // let mut stk_line = None;
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
        let open = Decimal::from_f32_retain(record.open).unwrap().round_dp(2);
        let high = Decimal::from_f32_retain(record.high).unwrap().round_dp(2);
        let low = Decimal::from_f32_retain(record.low).unwrap().round_dp(2);
        let close = Decimal::from_f32_retain(record.close).unwrap().round_dp(2);

        // println!("stk_line => {},{},{},{},{}",date,high,low,open,close);
        let stk_line = StockData::new(date, high, low, open, close);

        stock_data.push(stk_line);
    }

    Ok(stock_data)
}

// pub fn generate_stock_data_series(_limit: Option<u8>) -> Vec<StockData> {
//     let stock_data_series: Vec<StockData> = vec![];
//     // for number in 0..limit.unwrap_or(7) {
//     //     let number_plus = number + 1;

//     //     let stock_date = match number_plus {
//     //         number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
//     //         _ => format!("10-0{number_plus}-2022 00:00"),
//     //     };

//     //     let _stock_data = generate_stock_data(&stock_date);
//     //     // stock_data_series.push(stock_data);
//     //     // for stock in stock_data.iter() {
//     //     //     println!("{:?}", stock);
//     //     // }
//     // }

//     stock_data_series
// }

#[derive(Debug)]
pub struct StockInformation {
    company_name: String,
    #[allow(dead_code)]
    symbol: String,
    stock_data_series: Vec<StockData>,
}

impl StockInformation {
    pub fn new(company_name: String, symbol: String, stock_data_series: Vec<StockData>) -> Self {
        Self {
            company_name,
            symbol,
            stock_data_series,
        }
    }

    pub fn get_moving_averages(&self, ma_days: u16) -> Option<Vec<Decimal>> {
        if self.stock_data_series.len() == 0 {
            return None;
        }

        let mut moving_averages: Vec<Decimal> = vec![];
        let closing_prices = self.stock_data_series
            .iter()
            .map(|stock_data| stock_data.close)
            .collect::<Vec<Decimal>>();

        // No moving averages to be computed since current closing price series is not sufficient to build based upon ma_days parameters.
        if closing_prices.len() < ma_days.into() {
            return None;
        }

        let ma_days_idx_end = ma_days - 1;

        let ma_days_decimal = Decimal::from_u16(ma_days).unwrap();
        let mut sum = dec!(0.0);
        for x in 0..=ma_days_idx_end {
            let closing_price = &closing_prices[x.to_usize().unwrap()];
            sum = sum + closing_price;
        }

        let first_moving_average_day = sum / ma_days_decimal;
        moving_averages.push(first_moving_average_day.round_dp(2));

        if closing_prices.len() == ma_days.into() {
            return Some(moving_averages);
        }

        let mut idx: usize = 0;
        let mut tail_closing_day_idx: usize = (ma_days_idx_end + 1).to_usize().unwrap();

        while tail_closing_day_idx != closing_prices.len() {
            let previous_moving_average = &moving_averages[idx];
            let head_closing_day_price = &closing_prices[idx] / ma_days_decimal;
            let tail_closing_day_price = &closing_prices[tail_closing_day_idx] / ma_days_decimal;
            let current_moving_average =
                previous_moving_average - head_closing_day_price + tail_closing_day_price;
            moving_averages.push(current_moving_average.round_dp(2));

            idx += 1;
            tail_closing_day_idx += 1;
        }

        return Some(moving_averages);
    }

    pub fn show_chart(
        &self,
        ma_days: Vec<u16>,
        directory: Option<String>,
        height: Option<u32>,
        width: Option<u32>
    ) -> Result<bool, Box<dyn Error>> {
        let stock_data_series = &self.stock_data_series;
        if stock_data_series.len() == 0 {
            Err("Insufficient stock data series length")?;
        }

        if ma_days.len() > 3 {
            Err("Exceeded the limit of moving averages to plot")?;
        }

        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        let dir = directory.unwrap_or("chart_outputs".to_string());

        fs::create_dir_all(&dir)?;

        let filepath = format!("{}/{}_candlestick_chart_ohlcv.png", &dir, timestamp);
        let drawing_area = BitMapBackend::new(&filepath, (
            height.unwrap_or(1024),
            width.unwrap_or(768),
        )).into_drawing_area();

        drawing_area.fill(&WHITE)?;

        let candlesticks = stock_data_series
            .iter()
            .map(|stock_data| {
                CandleStick::new(
                    stock_data.date.date_naive(),
                    stock_data.open.to_f64().unwrap(),
                    stock_data.high.to_f64().unwrap(),
                    stock_data.low.to_f64().unwrap(),
                    stock_data.close.to_f64().unwrap(),
                    GREEN.filled(),
                    RED.filled(),
                    25
                )
            });

        let stock_data_series_last_day_idx = stock_data_series.len() - 1;

        let (from_date, to_date) = (
            stock_data_series[0].date.date_naive() - Duration::days(1),
            stock_data_series[stock_data_series_last_day_idx].date.date_naive() + Duration::days(1),
        );

        let mut chart_builder = ChartBuilder::on(&drawing_area);

        let min_low_price = stock_data_series
            .iter()
            .map(|stock_data| stock_data.low)
            .min()
            .unwrap();
        let max_high_price = stock_data_series
            .iter()
            .map(|stock_data| stock_data.high)
            .max()
            .unwrap();

        let x_spec = from_date..to_date;
        let y_spec = min_low_price.to_f64().unwrap()..max_high_price.to_f64().unwrap();
        let caption = format!("{} Stock Price Movement", &self.company_name);
        let font_style = ("sans-serif", 25.0).into_font();

        let mut chart = chart_builder
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(caption, font_style.clone())
            .build_cartesian_2d(x_spec, y_spec)?;

        chart.configure_mesh().light_line_style(&WHITE).draw()?;

        chart.draw_series(candlesticks)?;

        // Draw moving averages lines
        if ma_days.len() > 0 {
            // Parallel computed moving averages
            let moving_averages_2d: Vec<_> = ma_days
                .into_iter()
                .filter(|ma_day| ma_day > &&0)
                .map(|ma_day| {
                    let moving_averages = self.get_moving_averages(ma_day.clone());

                    match moving_averages {
                        Some(moving_averages) => {
                            return (ma_day, moving_averages);
                        }
                        None => {
                            return (ma_day, Vec::with_capacity(0));
                        }
                    }
                })
                .collect();

            for (idx, ma_tuple) in moving_averages_2d.iter().enumerate() {
                let (ma_day, moving_averages) = ma_tuple;
                let ma_line_data: Vec<(NaiveDate, f64)> = Vec::with_capacity(3);
                // let mut ma_line_data: Vec<NaiveDate> = Vec::with_capacity(3);
                let ma_len = moving_averages.len();

                // for i in 0..ma_len {
                //     // Let start moving average day at the day where adequate data has been formed.
                //     let ma_day = i + ma_day.to_usize().unwrap() - 1;
                //     ma_line_data.push((
                //         //HERE
                //         stock_data_series[ma_day].date.date_naive(),
                //         moving_averages[i].to_f64().unwrap(),
                //     ));
                // }

                if ma_len > 0 {
                    let chosen_color = [BLUE, PURPLE, ORANGE][idx];

                    let line_series_label = format!("SMA {}", &ma_day);

                    let legend = |color: RGBColor| {
                        move |(x, y)|
                            PathElement::new(
                                [
                                    (x, y),
                                    (x + 20, y),
                                ],
                                color
                            )
                    };

                    let sma_line = LineSeries::new(ma_line_data, chosen_color.stroke_width(2));

                    // Fill in moving averages line data series
                    chart
                        .draw_series(sma_line)
                        .unwrap()
                        .label(line_series_label)
                        .legend(legend(chosen_color));
                }

                // Display SMA Legend
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperLeft)
                    .label_font(font_style.clone())
                    .draw()
                    .unwrap();
            }
        }

        drawing_area
            .present()
            .expect(&format!("Cannot write into {:?}. Directory does not exists.", &dir));

        println!("Result has been saved to {}", filepath);

        Ok(true)
    }
}

fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
    // let stock_data_series = generate_stock_data_series(Some(14));
    let stock_data_series = generate_stock_data_series("blubb");
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series.expect("REASON")
    );

    let ma_days = vec![7, 2, 0];
    let chart = stock_information.show_chart(ma_days, None, None, None);

    match chart {
        Ok(_) => { assert!(true) }
        Err(err) => println!("Error in saving chart {:?}", err),
    }
}

fn main() {
    println!("main running");
    it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average();
}

// cargo run --example
// cargo run --example create_stock_report_6
