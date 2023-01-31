// This crate provides a function for getting circadian averages.



fn circadian_average(range: f64, data: Vec<f64>) -> (f64, f64) {
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
    let avg_x_pos = positions.iter().map(|(x, _)| x).sum::<f64>() / data.len() as f64;
    let avg_y_pos = positions.iter().map(|(_, y)| y).sum::<f64>() / data.len() as f64;
    // Get the angle of the average position
    let avg_angle = avg_y_pos.atan2(avg_x_pos);
    // Convert the angle to a value on the range
    let avg_value = (avg_angle / (2.0 * std::f64::consts::PI)) * range;
    // Get the confidence, which is the distance from the origin
    println!("Average position: {:?}", (avg_x_pos, avg_y_pos));
    let confidence = (avg_x_pos.powi(2) + avg_y_pos.powi(2)).sqrt() as f64;
    (avg_value, confidence)
}

fn main() {
    let data = vec![0.0, 0.0];
    let (avg, confidence) = circadian_average(4.0, data);
    println!("Average: {}, Error: {}", avg, confidence);
}
