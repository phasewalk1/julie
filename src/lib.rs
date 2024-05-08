pub trait ToJulian {
    fn to_julian(&self) -> f64;
}

impl ToJulian for chrono::DateTime<chrono::Utc> {
    fn to_julian(&self) -> f64 {
        julian_date(*self)
    }
}

pub fn julian_date_number(date: chrono::DateTime<chrono::Utc>) -> i32 {
    use chrono::Datelike;

    let y: i32 = date.year();
    let m: i32 = date.month() as i32;
    let d: i32 = date.day() as i32;

    let jdn = (1461 * (y + 4800 + (m - 14) / 12) / 4) + (367 * (m - 2 - 12 * ((m - 14) / 12)) / 12)
        - (3 * ((y + 4900 + (m - 14) / 12) / 100)) / 4
        + d
        - 32075;

    return jdn;
}

pub(crate) fn julian_date(date: chrono::DateTime<chrono::Utc>) -> f64 {
    use chrono::Timelike;

    let jdn = julian_date_number(date) as f64;
    let (hour, minute, second) = (
        date.hour() as f64,
        date.minute() as f64,
        date.second() as f64,
    );

    return jdn + (hour - 12.0) / 24.0 + minute / 1440.0 + second / 86400.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_julian_date() {
        let date = chrono::Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
        assert_eq!(julian_date(date), 2451544.5);
    }
}
