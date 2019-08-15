use std::thread;
use std::f64::consts::PI;
use chrono::{DateTime, NaiveDate, Datelike, Utc};

fn sin_deg(degrees: f64) -> f64 {
    (degrees * PI / 180.0).sin()
}

fn cos_deg(degrees: f64) -> f64 {
    (degrees *  PI / 180.0).cos()
}

fn tan_deg(degrees: f64) -> f64 {
    (degrees *  PI / 180.0).tan()
}

fn acos_deg(rad: f64) -> f64 {
    rad.acos() * 180.0 / PI
}

fn atan_deg(rad: f64) -> f64 {
    rad.atan() * 180.0 / PI
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

fn hours_to_hms(t: f64) -> (u32, u32, u32) {
    let hours = t as u32;
    let t = t - f64::from(hours);
    let minutes = (t * 60.0) as u32;
    let t = t - (f64::from(minutes) / 60.0);
    let seconds = (t * 3600.0) as u32;
    (hours, minutes, seconds)
}

fn day_of_year(y:i32, m: u32, d: u32) -> u32 {
    NaiveDate::from_ymd(y, m, d).ordinal()
}

fn approx_rise_time(y:i32, m: u32, d: u32, longitude_deg: f64) -> f64 {
    f64::from(day_of_year(y, m, d)) + (6.0 - longitude_deg / 15.0) / 24.0
}

fn approx_set_time(y:i32, m: u32, d: u32, longitude_deg: f64) -> f64 {
    f64::from(day_of_year(y, m, d)) + (18.0 - longitude_deg / 15.0) / 24.0
}

fn sun_mean_anomaly(approx_time: f64) -> f64 {
    0.9856 * approx_time - 3.289
}

fn sun_true_longitude(approx_time: f64) -> f64 {
    let m = sun_mean_anomaly(approx_time);
    let deg = m + (1.916 * sin_deg(m)) + (0.020 * sin_deg(2.0 * m)) + 282.634;
    constrain_degrees(deg)
}

fn sun_right_ascension_hours(approx_time: f64) -> f64 {
    let tl = sun_true_longitude(approx_time);
    let ra = atan_deg(0.91746 * tan_deg(tl));
    let ra = constrain_degrees(ra);
    let tl_quad = ((tl / 90.0) as i32) * 90;
    let ra_quad = ((ra / 90.0) as i32) * 90;
    (ra + f64::from(tl_quad - ra_quad)) / 15.0
}

fn sun_declination(approx_time: f64) -> (f64, f64) {
    let s = 0.39782 * sin_deg(sun_true_longitude(approx_time));
    let c = s.asin().cos();
    (s, c)
}

fn sun_local_hour_angle(approx_time: f64, latitude: f64, zenith: f64) -> f64 {
    let (s, c) = sun_declination(approx_time);
    let cos_h = (cos_deg(zenith) - (s * sin_deg(latitude))) / (c * cos_deg(latitude));
    let cos_h = if cos_h > 1.0 { 1.0 } else { cos_h }; // Sun never rises at this location on this date.
    let cos_h = if cos_h < -1.0 { -1.0 } else { cos_h }; // Sun nver sets at this location on this date.
    acos_deg(cos_h)
}

fn sun_local_rise_hour(approx_time: f64, latitude: f64, zenith: f64) -> f64 {
    (360.0 - sun_local_hour_angle(approx_time, latitude, zenith)) / 15.0
}

fn sun_local_set_hour(approx_time: f64, latitude: f64, zenith: f64) -> f64 {
    sun_local_hour_angle(approx_time, latitude, zenith) / 15.0
}

fn event_time(approx_time: f64, local_event_time: f64, longitude: f64) -> f64 {
    let ra = sun_right_ascension_hours(approx_time);
    let local_mean_time = local_event_time + ra - (0.06571 * approx_time) - 6.622;
    let ut = local_mean_time - longitude / 15.0;
    let ut = if ut < 0.0 { ut + 24.0 } else { ut };
    let ut = if ut > 24.0 { ut - 24.0 } else { ut };
    ut
}

pub fn sunrise(y:i32, m: u32, d: u32, latitude: f64, longitude: f64, zenith: f64) -> DateTime<Utc> {
    let at = approx_rise_time(y, m, d, longitude);
    let h = sun_local_rise_hour(at, latitude, zenith);
    let t = event_time(at, h, longitude);
    let (hr, min, sec) = hours_to_hms(t);
    let dt = NaiveDate::from_ymd(y, m, d).and_hms(hr, min, sec);
    DateTime::<Utc>::from_utc(dt, Utc)
}

pub fn sunset(y:i32, m: u32, d: u32, latitude: f64, longitude: f64, zenith: f64) -> DateTime<Utc> {
    let at = approx_set_time(y, m, d, longitude);
    let h = sun_local_set_hour(at, latitude, zenith);
    let t = event_time(at, h, longitude);
    let (hr, min, sec) = hours_to_hms(t);
    let dt = NaiveDate::from_ymd(y, m, d).and_hms(hr, min, sec);
    DateTime::<Utc>::from_utc(dt, Utc)
}

enum TaskResult {
    Early,
    Late,
    Executed
}

fn perform_task(start: DateTime<Utc>, end: DateTime<Utc>, current: DateTime<Utc>, action: &Fn()) -> TaskResult {
    if current < start {
        println!("early: {}", current);
        TaskResult::Early
    } else if current > end {
        println!("late: {}", current);
        TaskResult::Late
    } else {
        println!("executed: {}", current);
        action();
        TaskResult::Executed
    }
}

pub fn schedule(latitude: f64, longitude: f64, zenith: f64, period_millis: u64, action: &Fn()) {
    let now = Utc::now();
    let mut noon = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(12, 0, 0), Utc);
    let mut rise = sunrise(noon.year(), noon.month(), noon.day(), latitude, longitude, zenith);
    let mut set = sunset(noon.year(), noon.month(), noon.day(), latitude, longitude, zenith);
    let period = std::time::Duration::from_millis(period_millis);
    println!("Doing stuff from {} to {}", rise, set);
    loop {
        let res = perform_task(rise, set, Utc::now(), action);
        match res {
            TaskResult::Early => { },
            TaskResult::Late => {
                noon = noon + chrono::Duration::days(1);
                rise = sunrise(noon.year(), noon.month(), noon.day(), latitude, longitude, zenith);
                set = sunset(noon.year(), noon.month(), noon.day(), latitude, longitude, zenith);
                println!("Incremented date: doing stuff from {} to {}", rise, set);
            },
            TaskResult::Executed => { }
        }
        thread::sleep(period);
    }
}
