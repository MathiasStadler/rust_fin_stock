// hier weiter 
// generate_stock_data_series_ohlcv

// running into folder => /home/user/workspace_rust/rust_fin_stock
// cargo run --example creates_stock_report_1

use std::error::Error;
use std::fs;

use chrono::DateTime;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use plotters::prelude::BitMapBackend;
use plotters::prelude::CandleStick;
use plotters::prelude::ChartBuilder;
use plotters::prelude::IntoDrawingArea;
use plotters::prelude::LineSeries;
use plotters::prelude::PathElement;
use plotters::prelude::RGBColor;
use plotters::prelude::SeriesLabelPosition;
use rand::Rng;

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

fn generate_stock_data(date_string: &str) -> StockData {
    let base_stock_data_series = vec![
        (130.0600, 131.3700, 128.8300, 129.1500),
        (125.7900, 125.8500, 124.5200, 125.0100),
        (124.1000, 125.5800, 123.8300, 125.4400),
        (122.6200, 124.0000, 122.5700, 123.7600),
        (122.1900, 123.5200, 121.3018, 123.3700),
        (121.2400, 121.8500, 120.5400, 121.7700),
        (121.6400, 121.6500, 120.1000, 120.7700),
        (120.9400, 121.5800, 120.5700, 121.0500),
        (120.6400, 120.9800, 120.3700, 120.9500),
        (120.5400, 120.8500, 119.9200, 120.3300),
        (119.7600, 120.3500, 119.5400, 120.1900),
        (118.6300, 119.5400, 118.5800, 119.2800),
        (119.8100, 120.0200, 118.6400, 119.9300),
        (119.3900, 120.2300, 119.3700, 119.8900),
        (120.1000, 120.2300, 118.3800, 119.3600),
        (119.8600, 120.4300, 119.1500, 119.9700),
        (119.0600, 119.4800, 118.5200, 119.1900),
        (118.9500, 119.1085, 118.1000, 119.0200),
        (118.0700, 118.3200, 116.9600, 117.9400),
        (117.4400, 117.5800, 116.1300, 116.9300),
        (117.8750, 118.2100, 115.5215, 116.7700),
        (118.6200, 118.7050, 116.8500, 117.9100),
        (116.5600, 118.0100, 116.3224, 117.6600),
        (119.5000, 119.5900, 117.0400, 117.0500),
        (117.1350, 120.8200, 117.0900, 120.2200),
        (117.3900, 118.7500, 116.7100, 117.5200),
        (118.0900, 118.4400, 116.9900, 117.6500),
        (116.1700, 117.6100, 116.0500, 117.5700),
        (115.3400, 117.2500, 114.5900, 115.9100),
        (114.5400, 115.2000, 114.3300, 114.5900),
    ];

    let base_data_series_len = base_stock_data_series.len();

    let mut rng = rand::thread_rng();

    let high = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
        .unwrap()
        .round_dp(2);
    let low = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
        .unwrap()
        .round_dp(2);
    let open = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
        .unwrap()
        .round_dp(2);
    let close = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
        .unwrap()
        .round_dp(2);

    StockData::new(
        generate_utc_date_from_date_string(date_string),
        high,
        low,
        open,
        close,
    )
}

pub fn generate_stock_data_series(limit: Option<u8>) -> Vec<StockData> {
    let mut stock_data_series: Vec<StockData> = vec![];
    for number in 0..limit.unwrap_or(7) {
        let number_plus = number + 1;

        let stock_date = match number_plus {
            number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
            _ => format!("10-0{number_plus}-2022 00:00"),
        };

        let stock_data = generate_stock_data(&stock_date);
        stock_data_series.push(stock_data);
    }
    stock_data_series
}

fn generate_stock_data_ohlcv(date_string: &str) -> StockData {
    let base_stock_data_series = vec![
        (2024 - 07 - 12, 78.92, 82.11, 78.685, 81.55, 1629750),
        (2024 - 07 - 15, 82.1, 82.79, 81.24, 81.32, 1443184),
        (2024 - 07 - 16, 82.32, 85.92, 82.04, 85.82, 1076876),
        (2024 - 07 - 17, 84.37, 85.38, 83.31, 83.34, 753719),
        (2024 - 07 - 18, 83.64, 86.345, 82.33, 82.81, 900662),
        (2024 - 07 - 19, 82.7, 82.7, 80.86, 81.15, 771416),
        (2024 - 07 - 22, 82.11, 82.15, 79.78, 81.72, 903883),
        (2024 - 07 - 23, 80.15, 80.62, 79.365, 79.55, 1082063),
        (2024 - 07 - 24, 78.88, 79.852, 76.42, 76.5, 1129719),
        (2024 - 07 - 25, 77.07, 80.12, 76.875, 78.48, 781867),
        (2024 - 07 - 26, 80.71, 83.73, 80.405, 82.5, 1012631),
        (2024 - 07 - 29, 82.83, 84.25, 82.395, 83.59, 904321),
        (2024 - 07 - 30, 83.88, 84.82, 83.08, 84.5, 859673),
        (2024 - 07 - 31, 85.0, 86.56, 83.41, 83.63, 899712),
        (2024 - 08 - 01, 84.17, 85.14, 79.85, 80.43, 1071646),
        (2024 - 08 - 02, 78.12, 78.12, 75.3, 77.26, 925535),
        (2024 - 08 - 05, 72.63, 76.65, 72.2, 75.45, 1054175),
        (2024 - 08 - 06, 74.96, 78.18, 74.28, 76.56, 1684381),
        (2024 - 08 - 07, 61.04, 62.7, 58.68, 60.07, 6830593),
        (2024 - 08 - 08, 61.01, 62.49, 59.46, 60.7, 2552222),
        (2024 - 08 - 09, 60.55, 61.86, 60.0, 60.7, 1836974),
        (2024 - 08 - 12, 60.42, 61.19, 58.91, 59.17, 1663846),
        (2024 - 08 - 13, 59.49, 61.68, 59.43, 61.42, 1494238),
        (2024 - 08 - 14, 61.94, 62.62, 60.365, 60.46, 1017821),
        (2024 - 08 - 15, 61.73, 64.3, 61.28, 63.75, 1678198),
        (2024 - 08 - 16, 63.33, 64.495, 63.09, 63.4, 1518835),
        (2024 - 08 - 19, 64.03, 64.9, 63.45, 64.0, 927182),
        (2024 - 08 - 20, 64.17, 65.27, 63.29, 64.11, 839691),
        (2024 - 08 - 21, 64.34, 65.57, 64.21, 65.43, 771763),
        (2024 - 08 - 22, 65.47, 65.97, 63.93, 64.08, 723877),
        (2024 - 08 - 23, 64.7, 67.0, 64.425, 66.865, 64208),
    ];

    let base_data_series_len = base_stock_data_series.len();

    let mut rng = rand::thread_rng();

    let fmt = "%Y-%m-%d";
    let tmp_date_string = base_stock_data_series[rng.gen_range(0..base_data_series_len)].1;
    println!("date_string => {}", tmp_date_string);
    // let date = DateTime::parse_from_str(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1, fmt)
    // .unwrap();

    // let date =  (0..base_data_series_len)].0);
    let high = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
        .unwrap()
        .round_dp(2);
    let low = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
        .unwrap()
        .round_dp(2);
    let open = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
        .unwrap()
        .round_dp(2);
    let close = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].4)
        .unwrap()
        .round_dp(2);

    StockData::new(
        generate_utc_date_from_date_string(date_string),
        high,
        low,
        open,
        close,
    )
}

pub fn generate_stock_data_series_ohlcv(limit: Option<u8>) -> Vec<StockData> {
    let mut stock_data_series: Vec<StockData> = vec![];
    for number in 0..limit.unwrap_or(7) {
        let number_plus = number + 1;

        let stock_date = match number_plus {
            number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
            _ => format!("10-0{number_plus}-2022 00:00"),
        };

        let stock_data = generate_stock_data(&stock_date);
        stock_data_series.push(stock_data);
    }
    stock_data_series
}
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
        let closing_prices = self
            .stock_data_series
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
        width: Option<u32>,
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
        let drawing_area =
            BitMapBackend::new(&filepath, (height.unwrap_or(1024), width.unwrap_or(768)))
                .into_drawing_area();

        drawing_area.fill(&WHITE)?;

        let candlesticks = stock_data_series.iter().map(|stock_data| {
            CandleStick::new(
                stock_data.date.date_naive(),
                stock_data.open.to_f64().unwrap(),
                stock_data.high.to_f64().unwrap(),
                stock_data.low.to_f64().unwrap(),
                stock_data.close.to_f64().unwrap(),
                GREEN.filled(),
                RED.filled(),
                25,
            )
        });

        let stock_data_series_last_day_idx = stock_data_series.len() - 1;

        let (from_date, to_date) = (
            stock_data_series[0].date.date_naive() - Duration::days(1),
            stock_data_series[stock_data_series_last_day_idx]
                .date
                .date_naive()
                //.time()
                + Duration::days(1),
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
                        Some(moving_averages) => return (ma_day, moving_averages),
                        None => return (ma_day, Vec::with_capacity(0)),
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
                        move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color)
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

        drawing_area.present().expect(&format!(
            "Cannot write into {:?}. Directory does not exists.",
            &dir
        ));

        println!("Result has been saved to {}", filepath);

        Ok(true)
    }
}

fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
    let stock_data_series = generate_stock_data_series_ohlcv(Some(14));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let ma_days = vec![7, 2, 0];
    let chart = stock_information.show_chart(ma_days, None, None, None);

    match chart {
        Ok(_) => {
            assert!(true)
        }
        Err(err) => println!("Error in saving chart {:?}", err),
    }
}

fn main() {
    println!("main running");
    it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average();
}


// cargo run --example
// cargo run --example create_stock_report_2