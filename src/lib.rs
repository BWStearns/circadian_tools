use num_traits::Float;
use std::{f64::consts::TAU, fmt::Debug};

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono")]
pub use crate::chrono::avg_time_of_day;

pub fn circadian_average<I, F>(range: F, data: I) -> (F, F)
where
    F: Float + Debug,
    I: Iterator<Item = F>,
{
    let mut len = 0;
    let mut x_pos_sum = F::zero();
    let mut y_pos_sum = F::zero();

    // This unwrap is reasonable as this can not be done if F can't be represented as 2PI
    let tau = F::from(TAU).unwrap();
    let tau_over_range = tau / range;

    for x in data {
        debug_assert!(x >= F::zero(), "Input data must be positive");
        len += 1;
        // Get X, Y position of each data point on a circle with a perimeter of range
        let angle = x * tau_over_range;
        let (s, c) = angle.sin_cos();
        x_pos_sum = x_pos_sum + c;
        y_pos_sum = y_pos_sum + s;
    }

    let avg_x_pos = x_pos_sum / F::from(len).unwrap();
    let avg_y_pos = y_pos_sum / F::from(len).unwrap();
    // Get the angle of the average position
    let mut avg_angle = avg_y_pos.atan2(avg_x_pos);
    if avg_angle < F::zero() {
        avg_angle = avg_angle + tau;
    }
    // Convert the angle to a value on the range
    let avg_value = avg_angle / tau_over_range;
    // Get the confidence, which is the distance of the average from the origin
    let confidence = (avg_x_pos.powi(2) + avg_y_pos.powi(2)).sqrt();
    (avg_value, confidence)
}

// This function implements the same algorithm as circadian_average, but it uses a running average
// instead of a sum. This is useful for when the data is too large to fit in memory.
pub fn safe_circadian_average<I, F>(range: F, data: I) -> (F, F)
where
    F: Float + Debug,
    I: Iterator<Item = F>,
{
    let mut len: u32 = 0;
    let mut avg_x_pos = F::zero();
    let mut avg_y_pos = F::zero();

    // This unwrap is reasonable as this can not be done if F can't be represented as 2PI
    let tau = F::from(TAU).unwrap();
    let tau_over_range = tau / range;

    for x in data {
        debug_assert!(x >= F::zero(), "Input data must be positive");
        len += 1;
        // Get X, Y position of each data point on a circle with a perimeter of range
        let angle = x * tau_over_range;
        let (s, c) = angle.sin_cos();
        // Get the running average of the X, Y positions
        avg_x_pos = avg_x_pos + (c - avg_x_pos) / F::from(len).unwrap();
        avg_y_pos = avg_y_pos + (s - avg_y_pos) / F::from(len).unwrap();
    }

    // Get the angle of the average position
    let mut avg_angle = avg_y_pos.atan2(avg_x_pos);
    if avg_angle < F::zero() {
        avg_angle = avg_angle + tau;
    }
    // Convert the angle to a value on the range
    let avg_value = avg_angle / tau_over_range;
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
    #[should_panic(expected = "Input data must be positive")]
    fn negative_input_fails() {
        let data = vec![-1.0];
        let _ = circadian_average(4.0, data.into_iter());
    }

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
        assert!(approx_eq!(f64, avg, 4.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, FRAC_1_SQRT_2, epsilon = 0.0001));
    }

    #[test]
    fn test_circadian_average_split() {
        let data = vec![1.0, 2.0];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
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

    #[test]
    fn test_average_in_lower_left_quadrant() {
        let data = vec![2.0, 3.0];
        let (avg, confidence) = circadian_average(4.0, data.into_iter());
        assert!(approx_eq!(f64, avg, 2.5, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, FRAC_1_SQRT_2, epsilon = 0.0001));
    }

    #[test]
    fn avg_count() {
        const FACTOR: f64 = 607.0;
        let data: Vec<f64> = vec![
            514.0, 176.0, 64.0, 249.0, 415.0, 455.0, 221.0, 375.0, 477.0, 464.0, 421.0, 32.0, 40.0,
            496.0, 534.0, 134.0,
        ];
        let inputs = data.into_iter().map(|x| x);
        let (avg, confidence) = circadian_average(FACTOR, inputs);
        assert_eq!(avg, 498.7531532014195);
        assert_eq!(confidence, 0.23138448716890458)
    }

    #[test]
    fn test_safe_circadian_average_unanimous() {
        let data = vec![0.0, 0.0];
        let (avg, confidence) = safe_circadian_average(4.0, data.into_iter());
        assert_eq!(avg, 0.0);
        assert_eq!(confidence, 1.0);
    }

    #[test]
    fn test_safe_circadian_crossing_zero() {
        let data = vec![0.5, 3.5];
        let (avg, confidence) = safe_circadian_average(4.0, data.into_iter());
        assert!(approx_eq!(f64, avg, 4.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, FRAC_1_SQRT_2, epsilon = 0.0001));
    }

    #[test]
    fn test_with_reasonably_sized_numbers() {
        let data: Vec<f32> = vec![1000.0, 2000.0];
        let (avg, confidence) = safe_circadian_average(4000.0, data.into_iter());
        assert_eq!(avg, 1500_f32);
        assert!(approx_eq!(f32, confidence, FRAC_1_SQRT_2 as f32, epsilon = 0.0001));
    }

    #[test]
    fn test_safe_circadian_with_big_numbers() {
        // This test is to make sure that the running average doesn't overflow with
        // large numbers.
        let data: Vec<f32> = vec![(f32::MAX / 2_f32), (f32::MAX / 2_f32), (f32::MAX / 2_f32)];
        let (avg, _confidence) = safe_circadian_average((f32::MAX), data.into_iter());
        assert_eq!(avg, (f32::MAX / 2_f32));
    }

    #[test]
    fn test_that_averaging_max_works() {
        let avg = f32::MAX + (f32::MAX-f32::MAX) / 1.0_f32;
        assert!(approx_eq!(f32, avg, f32::MAX, epsilon = 0.0001));
    }
}
