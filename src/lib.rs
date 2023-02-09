use std::f64::consts::TAU;

use num_traits::Float;

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono")]
pub use crate::chrono::avg_time_of_day;

pub fn circadian_average<I, F>(range: F, data: I) -> (F, F)
where
    F: Float,
    I: Iterator<Item = F>,
{
    let mut len = 0;
    let mut x_pos_sum = F::zero();
    let mut y_pos_sum = F::zero();
    
    // This unwrap is reasonable as this can not be done if F can't be represented as 2PI
    let tau = F::from(TAU).unwrap();
    let dist_to_angle = tau / range;

    for x in data {
        len += 1;
        // Get X, Y position of each data point on a circle with a perimeter of range
        let angle = x * dist_to_angle;
        let (s, c) = angle.sin_cos();
        x_pos_sum = x_pos_sum + c;
        y_pos_sum = y_pos_sum + s;
    }

    let avg_x_pos = x_pos_sum / F::from(len).unwrap();
    let avg_y_pos = y_pos_sum / F::from(len).unwrap();

    // Get the angle of the average position
    let avg_angle = avg_y_pos.atan2(avg_x_pos);
    // Convert the angle to a value on the range
    let avg_value = avg_angle / dist_to_angle;
    // Get the confidence, which is the distance of the average from the origin
    let confidence = (avg_x_pos.powi(2) + avg_y_pos.powi(2)).sqrt();
    (avg_value, confidence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_circadian_average_unanimous() {
        let data = vec![0.0, 0.0];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
        assert_eq!(avg, 0.0);
        assert_eq!(confidence, 1.0);
    }

    #[test]
    fn test_circadian_crossing_zero() {
        let data = vec![0.5, 3.5];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
        assert!(approx_eq!(f64, avg, 0.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, FRAC_1_SQRT_2, epsilon = 0.0001));
    }

    #[test]
    fn test_circadian_average_split() {
        let data = vec![1.0, 2.0];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
        println!("Average: {avg}, Error: {confidence}");
        assert_eq!(avg, 1.5);
        assert!(approx_eq!(f64, confidence, FRAC_1_SQRT_2, epsilon = 0.0001));
    }

    #[test]
    fn test_circadian_average_even_split() {
        let data = vec![0.0, 2.0];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
        assert!(approx_eq!(f64, avg, 1.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, 0.0, epsilon = 0.0001));
    }
}
