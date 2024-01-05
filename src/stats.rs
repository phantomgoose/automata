use std::collections::VecDeque;

use macroquad::color::{Color, LIGHTGRAY, WHITE};
use macroquad::shapes::{draw_line, draw_rectangle};

const BUCKET_SIZE_MILLISECONDS: i32 = 100;
const CHART_WINDOW_SECONDS: usize = 10;
const BUCKET_COUNT: usize = CHART_WINDOW_SECONDS * (1000 / BUCKET_SIZE_MILLISECONDS) as usize;
const CHART_HEIGHT: f32 = 50.;
const CHART_WIDTH: f32 = BUCKET_COUNT as f32;
// 1 pixel per bucket
const CHART_BORDER_COLOR: Color = LIGHTGRAY;
const CHART_BORDER_THICCNESS: f32 = 1.;
const CHART_LINE_COLOR: Color = WHITE;
const CHART_LINE_THICCNESS: f32 = 2.;

pub struct DataPoint {
    timestamp_millis: i32,
    value: f32,
}

impl DataPoint {
    pub fn new(timestamp_millis: i32, value: f32) -> Self {
        Self {
            timestamp_millis,
            value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DataPointBucket {
    clamped_timestamp_millis: i32,
    sum: f32,
    count: i32,
}

impl DataPointBucket {
    fn avg(&self) -> f32 {
        self.sum / (self.count as f32)
    }

    fn store(&mut self, point: &DataPoint) {
        self.count += 1;
        self.sum += &point.value;
    }
}

impl From<DataPoint> for DataPointBucket {
    fn from(dp: DataPoint) -> Self {
        let DataPoint {
            timestamp_millis,
            value,
        } = dp;

        // round down to nearest bucket size in milliseconds
        let clamped_timestamp_millis =
            (timestamp_millis / BUCKET_SIZE_MILLISECONDS) * BUCKET_SIZE_MILLISECONDS;

        Self {
            clamped_timestamp_millis,
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
            self.series.push_back(DataPointBucket::from(data_point));
            return;
        }

        // check if new ts is within X millis of the first value in the bucket
        let last_bucket = last_bucket.expect("Bucket should not be None at this point");

        let bucket_start_ts = last_bucket.clamped_timestamp_millis;

        let DataPoint {
            timestamp_millis, ..
        } = data_point;
        assert!(
            timestamp_millis >= bucket_start_ts,
            "Only chronologically ordered points are supported."
        );

        let time_diff = timestamp_millis - bucket_start_ts;

        // if the new point falls outside the last bucket, append a new bucket
        if time_diff >= BUCKET_SIZE_MILLISECONDS {
            self.series.push_back(DataPointBucket::from(data_point));
        }
        // otherwise store the new point in the last bucket
        else {
            last_bucket.store(&data_point);
        }

        // remove old buckets
        while self.series.len() > BUCKET_COUNT {
            self.series.pop_front();
        }
    }

    pub fn display(&self, x: f32, y: f32) {
        // x, y is the upper left corner

        // draw border
        // left side
        draw_line(
            x,
            y,
            x,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            CHART_BORDER_THICCNESS,
            CHART_BORDER_COLOR,
        );
        // bottom
        draw_line(
            x,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            x + CHART_WIDTH,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            CHART_BORDER_THICCNESS,
            CHART_BORDER_COLOR,
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
                CHART_LINE_THICCNESS,
                CHART_LINE_THICCNESS,
                CHART_LINE_COLOR,
            );
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_series_records_points() {
        let mut ts = TimeSeries::new();
        ts.record(DataPoint::new(0, 1.));
        ts.record(DataPoint::new(100, 2.));
        ts.record(DataPoint::new(200, 3.));

        assert_eq!(
            ts.series,
            VecDeque::from(vec![
                DataPointBucket {
                    clamped_timestamp_millis: 0,
                    sum: 1.,
                    count: 1,
                },
                DataPointBucket {
                    clamped_timestamp_millis: 100,
                    sum: 2.,
                    count: 1,
                },
                DataPointBucket {
                    clamped_timestamp_millis: 200,
                    sum: 3.,
                    count: 1,
                },
            ])
        );
    }

    #[test]
    fn time_series_handles_same_bucket_points() {
        let mut ts = TimeSeries::new();
        ts.record(DataPoint::new(0, 1.));
        ts.record(DataPoint::new(50, 2.));
        ts.record(DataPoint::new(75, 3.));

        assert_eq!(
            ts.series,
            VecDeque::from(vec![DataPointBucket {
                clamped_timestamp_millis: 0,
                sum: 6.,
                count: 3,
            },])
        );
    }

    #[test]
    fn time_series_is_finite() {
        let mut ts = TimeSeries::new();

        for idx in 0..(BUCKET_COUNT + 1000) {
            ts.record(DataPoint::new(idx as i32 * BUCKET_SIZE_MILLISECONDS, 1.));
        }

        assert_eq!(ts.series.len(), BUCKET_COUNT);
    }
}
