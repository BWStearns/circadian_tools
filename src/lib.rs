// This crate provides a function for getting circadian averages.

pub fn circadian_average(range: f64, data: Vec<f64>) -> (f64, f64) {
    let num_samples = data.len();
    // Get X, Y position of each data point on a circle with a perimeter of range
    let positions: Vec<(f64, f64)> = data
        .iter()
        .map(|x| {
            let angle = (x / range) * 2.0 * std::f64::consts::PI;
            (angle.cos(), angle.sin())
        })
        .collect();
    println!("Positions: {:?}", positions);
    let avg_x_pos = positions.iter().map(|(x, _)| x).sum::<f64>() / num_samples as f64;
    let avg_y_pos = positions.iter().map(|(_, y)| y).sum::<f64>() / num_samples as f64;
    // Get the angle of the average position
    let avg_angle = avg_y_pos.atan2(avg_x_pos);
    // Convert the angle to a value on the range
    let avg_value = (avg_angle / (2.0 * std::f64::consts::PI)) * range;
    // Get the confidence, which is the distance of the average from the origin
    println!("Average position: {:?}", (avg_x_pos, avg_y_pos));
    let confidence = (avg_x_pos.powi(2) + avg_y_pos.powi(2)).sqrt() as f64;
    (avg_value, confidence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_circadian_average_unanimous() {
        let data = vec![0.0, 0.0];
        let (avg, confidence) = circadian_average(4.0, data);
        assert_eq!(avg, 0.0);
        assert_eq!(confidence, 1.0);
    }

    #[test]
    fn test_circadian_crossing_zero() {
        let data = vec![0.5, 3.5];
        let (avg, confidence) = circadian_average(4.0, data);
        assert!(approx_eq!(f64, avg, 0.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, 0.70710678, epsilon = 0.0001));
    }

    #[test]
    fn test_circadian_average_split() {
        let data = vec![1.0, 2.0];
        let (avg, confidence) = circadian_average(4.0, data);
        println!("Average: {}, Error: {}", avg, confidence);
        assert_eq!(avg, 1.5);
        assert!(approx_eq!(f64, confidence, 0.70710678, epsilon = 0.0001));
    }

    #[test]
    fn test_circadian_average_even_split() {
        let data = vec![0.0, 2.0];
        let (avg, confidence) = circadian_average(4.0, data);
        assert!(approx_eq!(f64, avg, 1.0, epsilon = 0.0001));
        assert!(approx_eq!(f64, confidence, 0.0, epsilon = 0.0001));
    }
}
