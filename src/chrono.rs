// This crate provides a function for getting circadian averages.
use chrono::{self, NaiveTime, Timelike};

pub fn avg_time_of_day<I, T>(data: I) -> NaiveTime
where
    I: Iterator<Item = T>,
    T: Timelike,
{
    let data = data
        .map(|x| x.num_seconds_from_midnight() as f64);

    let avg_time = crate::circadian_average(86400.0, data).0;
    NaiveTime::from_num_seconds_from_midnight_opt(avg_time as u32, 0).unwrap()
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
}
