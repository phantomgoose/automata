use std::collections::VecDeque;

use macroquad::color::{Color, LIGHTGRAY, WHITE};
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::text::draw_text;

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
const CHART_LEGEND_FONT_SIZE: f32 = 14.;

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
    timestamp_millis: i32,
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
    fn from(data_point: DataPoint) -> Self {
        let DataPoint {
            timestamp_millis,
            value,
        } = data_point;

        // round down to nearest bucket size in milliseconds
        let timestamp_millis =
            (timestamp_millis / BUCKET_SIZE_MILLISECONDS) * BUCKET_SIZE_MILLISECONDS;

        Self {
            timestamp_millis,
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

        let bucket_start_ts = last_bucket.timestamp_millis;

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

    /// Draw a simple line chart for the average values of the points in the time series. x, y is the upper left corner.
    pub fn display(&self, x: f32, y: f32, current_val_label: &str) {
        let mut points: [Option<f32>; BUCKET_COUNT] = [None; BUCKET_COUNT];
        let mut max_val = 0.; // used for scaling the chart later

        assert!(
            self.series.len() <= BUCKET_COUNT,
            "Expected stored buckets to not exceed the max bucket count"
        );

        for (idx, bucket) in self.series.iter().enumerate() {
            let bucket_avg = bucket.avg();

            points[idx] = Some(bucket_avg);

            if bucket_avg > max_val {
                max_val = bucket_avg;
            }
        }

        // legend for max value
        draw_text(
            format!("Max: {}", max_val as i32).as_str(),
            x,
            y - CHART_LEGEND_FONT_SIZE / 2.,
            CHART_LEGEND_FONT_SIZE,
            CHART_BORDER_COLOR,
        );

        // left border
        draw_line(
            x,
            y,
            x,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            CHART_BORDER_THICCNESS,
            CHART_BORDER_COLOR,
        );

        // bottom border
        draw_line(
            x,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            x + CHART_WIDTH,
            y + CHART_HEIGHT + CHART_LINE_THICCNESS,
            CHART_BORDER_THICCNESS,
            CHART_BORDER_COLOR,
        );

        let mut prev_x = 0.;
        let mut prev_y = 0.;
        for (idx, point) in points.iter().enumerate() {
            if point.is_none() {
                // no more valid points in the array
                break;
            };

            let point = point.unwrap();
            let point_height = point / max_val * CHART_HEIGHT;

            let point_x = x + idx as f32;
            let point_y = y + (CHART_HEIGHT - point_height);

            // draw the first point as a simple rectangle
            if idx == 0 {
                draw_rectangle(
                    point_x,
                    point_y,
                    CHART_LINE_THICCNESS,
                    CHART_LINE_THICCNESS,
                    CHART_LINE_COLOR,
                );

                prev_x = point_x;
                prev_y = point_y;

                continue;
            }

            // draw the remaining points as lines connected to the previous point
            draw_line(
                prev_x,
                prev_y,
                point_x,
                point_y,
                CHART_LINE_THICCNESS,
                CHART_LINE_COLOR,
            );

            prev_x = point_x;
            prev_y = point_y;
        }

        // draw the legend for the last point
        if let Some(Some(point)) = points.last() {
            draw_text(
                format!("{} {}", *point as i32, current_val_label).as_str(),
                x + CHART_WIDTH + 1.0,
                prev_y,
                CHART_LEGEND_FONT_SIZE,
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
                    timestamp_millis: 0,
                    sum: 1.,
                    count: 1,
                },
                DataPointBucket {
                    timestamp_millis: 100,
                    sum: 2.,
                    count: 1,
                },
                DataPointBucket {
                    timestamp_millis: 200,
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
                timestamp_millis: 0,
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
