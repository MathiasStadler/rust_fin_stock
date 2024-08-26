mod creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average;

use plotters::{
    prelude::{
        BitMapBackend, CandleStick, ChartBuilder, IntoDrawingArea, PathElement, SeriesLabelPosition,
    },
    series::LineSeries,
    style::{
        full_palette::{ORANGE, PURPLE},
        Color, IntoFont, RGBColor, BLUE, GREEN, RED, WHITE,
    },
};

#[derive(Debug)]
pub struct StockInformation {
    company_name: String,
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
                let mut ma_line_data: Vec<(NaiveDate, f64)> = Vec::with_capacity(3);
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

fn generate_stock_data_series(limit: Option<u8>) -> Vec<StockData> {
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

fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
    let stock_data_series = generate_stock_data_series(Some(14));
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

fn main(){

    println!("main running");
    it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average();
}