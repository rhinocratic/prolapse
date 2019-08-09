use std::f64::consts::PI;
use chrono::{NaiveDate, Datelike};

pub const OFFICIAL: f64 = (90.0 + 5.0 / 6.0);
pub const CIVIL: f64 = 90.0;
pub const NAUTICAL: f64 = 102.0;
pub const ASTRONOMICAL: f64 = 108.0;

fn sin_deg(degrees: f64) -> f64 {
    (degrees * PI / 180.0).sin()
}

fn cos_deg(degrees: f64) -> f64 {
    (degrees *  PI / 180.0).cos()
}

fn tan_deg(degrees: f64) -> f64 {
    (degrees *  PI / 180.0).tan()
}

fn atan_deg(rad: f64) -> f64 {
    rad.atan() * 180.0 / PI
}

fn acos_deg(rad: f64) -> f64 {
    rad.acos() * 180.0 / PI
}

fn constrain_degrees(degrees: f64) -> f64 {
    if degrees >= 0.0 && degrees < 360.0 {
        degrees
    } else if degrees < 0.0 {
        degrees + 360.0
    } else {
        degrees - 360.0
    }
}

fn constrain_hours(hours: f64) -> f64 {
    if hours >= 0.0 && hours < 24.0 {
        hours
    } else if hours < 0.0 {
        hours + 24.0
    } else {
        hours - 24.0
    }
}

fn day_of_year(y:i32, m: u32, d: u32) -> u32 {
    NaiveDate::from_ymd(y, m, d).ordinal()
}

fn cos_hour_angle(observer_latitude: f64, sun_declination: f64) -> f64 {
    - observer_latitude.tan() * sun_declination.tan()
}

fn radians_to_time(angle: f64) -> (u32, u32, u32) {
    degrees_to_time(angle * 180.0 / PI)
}

fn degrees_to_time(angle: f64) -> (u32, u32, u32) {
    let angle = if angle < 0.0 { angle + 360.0 } else { angle };
    let hours = (angle / 15.0) as u32;
    let angle = angle - f64::from(15 * hours);
    let minutes = (angle * 4.0) as u32;
    let angle = angle - f64::from(minutes) / 4.0;
    let seconds = (angle * 240.0).round() as u32;
    (hours, minutes, seconds)
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_year() {
        assert_eq!(61, day_of_year(2000, 3, 1));
        assert_eq!(60, day_of_year(1900, 3, 1));
        assert_eq!(366, day_of_year(2008, 12, 31));
    }

    #[test]
    fn test_degrees_to_time() {
        assert_eq!((6, 0, 0), degrees_to_time(90.0));
        assert_eq!((18, 0, 0), degrees_to_time(-90.0));
        assert_eq!((0, 20, 0), degrees_to_time(5.0));
        assert_eq!((0, 0, 45), degrees_to_time(0.25 * 0.75));
        assert_eq!((6, 20, 45), degrees_to_time(90.0 + 5.0 + 0.25 * 0.75));
    }
}
