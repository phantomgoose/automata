use std::collections::VecDeque;

use macroquad::color::{Color, LIGHTGRAY, WHITE};
use macroquad::shapes::{draw_line, draw_rectangle};

const BUCKET_SIZE_SECS: f32 = 0.1;
const CHART_MAX_SECONDS: usize = 10;
const MAX_BUCKET_COUNT: usize = CHART_MAX_SECONDS * (1. / BUCKET_SIZE_SECS) as usize;

pub struct DataPoint {
    timestamp_ms: f32,
    value: i32,
}

impl DataPoint {
    pub fn new(timestamp_ms: f32, value: i32) -> Self {
        Self {
            timestamp_ms,
            value,
        }
    }
}

#[derive(Debug)]
pub struct DataPointBucket {
    first_ts_ms: f32,
    sum: i32,
    count: i32,
}

impl DataPointBucket {
    fn avg(&self) -> f32 {
        (self.sum as f32) / (self.count as f32)
    }

    fn store(&mut self, point: &DataPoint) {
        self.count += 1;
        self.sum += &point.value;
    }
}

impl From<DataPoint> for DataPointBucket {
    fn from(dp: DataPoint) -> Self {
        let DataPoint {
            timestamp_ms,
            value,
        } = dp;

        Self {
            first_ts_ms: timestamp_ms,
            sum: value,
            count: 1,
        }
    }
}

pub struct TimeSeries {
    series: VecDeque<DataPointBucket>,
}

impl TimeSeries {
    pub fn new() -> Self {
        Self {
            series: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = TimeSeries::new();
    }

    pub fn record(&mut self, data_point: DataPoint) {
        let last_bucket = self.series.iter_mut().last();
        if last_bucket.is_none() {
            // no elements added yet
            self.series.push_back(data_point.into());
            return;
        }

        // check if new ts is within X millis of the first value in the bucket
        let mut last_bucket = last_bucket.expect("Bucket should not be None at this point");

        let bucket_start_ts = last_bucket.first_ts_ms;

        let DataPoint {
            timestamp_ms,
            value,
        } = data_point;
        assert!(
            timestamp_ms > bucket_start_ts,
            "Only chronologically ordered points are supported."
        );

        let time_diff = timestamp_ms - last_bucket.first_ts_ms;

        // if the new point falls outside the last bucket, append a new bucket
        if time_diff > BUCKET_SIZE_SECS {
            self.series.push_back(data_point.into());
        }
        // otherwise store the new point in the last bucket
        else {
            last_bucket.store(&data_point);
        }

        // remove old buckets
        while self.series.len() > MAX_BUCKET_COUNT {
            self.series.pop_front();
        }
    }

    pub fn display(&self, x: f32, y: f32) {
        const CHART_HEIGHT: f32 = 50.;
        const CHART_WIDTH: f32 = MAX_BUCKET_COUNT as f32; // 1 pixel per bucket
        const BORDER_COLOR: Color = LIGHTGRAY;
        const BORDER_THICCNESS: f32 = 1.;
        const LINE_COLOR: Color = WHITE;
        const LINE_THICCNESS: f32 = 1.75;

        // x, y is the upper left corner

        // draw border
        // left side
        draw_line(x, y, x, y + CHART_HEIGHT, BORDER_THICCNESS, BORDER_COLOR);
        // bottom
        draw_line(
            x,
            y + CHART_HEIGHT,
            x + CHART_WIDTH,
            y + CHART_HEIGHT,
            BORDER_THICCNESS,
            BORDER_COLOR,
        );

        // draw the averages
        let points: Vec<f32> = self.series.iter().map(|b| b.avg()).collect();

        // TODO: optimize this
        let mut max_val = 0.;
        for point in &points {
            if *point > max_val {
                max_val = *point;
            }
        }

        for (idx, point) in points.into_iter().enumerate() {
            let point_height = point / max_val * CHART_HEIGHT;
            draw_rectangle(
                x + idx as f32,
                y + (CHART_HEIGHT - point_height),
                1.,
                1.,
                LINE_COLOR,
            );
        }
    }
}
