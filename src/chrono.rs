// This crate provides a function for getting circadian averages.
use chrono::{self, NaiveTime, Timelike, Weekday};
use num_traits::FromPrimitive;

pub fn avg_time_of_day<I, T>(data: I) -> NaiveTime
where
    I: Iterator<Item = T>,
    T: Timelike,
{
    let data = data.map(|x| x.num_seconds_from_midnight() as f64);
    let avg_time = crate::circadian_average(86400.0, data).0;
    NaiveTime::from_num_seconds_from_midnight_opt(avg_time as u32, 0).unwrap()
}

pub fn avg_weekday(data: impl Iterator<Item = Weekday>) -> Weekday {
    let data = data
        .map(|x| x.num_days_from_sunday() as f64);

    let (avg_day, confidence) = crate::circadian_average(7.0, data);
    let avg_day = avg_day.round() as u32;
    Weekday::from_u32(avg_day).unwrap()
}

#[cfg(test)]
mod avg_time_tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_avg_time_of_day() {
        let data = vec![
            NaiveTime::from_hms_opt(1, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
        ];
        let avg_time = avg_time_of_day(data.into_iter());
        assert_eq!(avg_time, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    }

    #[test]
    fn test_avg_day_of_week() {
        let data = vec![
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
            Weekday::Fri,
        ];
        let avg_day = avg_weekday(data.into_iter());
        assert_eq!(avg_day, Weekday::Fri);
    }
}
