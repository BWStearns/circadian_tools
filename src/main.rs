// This crate provides a function for getting circadian averages.



fn circadian_average(range: f64, data: Vec<f64>) -> (f64, f64) {
    // Get X, Y position of each data point on a circle with a perimeter of range
    let positions: Vec<(f64, f64)> = data
        .iter()
        .map(|x| {
            let angle = (x / range) * 2.0 * std::f64::consts::PI;
            (angle.cos(), angle.sin())
        })
        .collect();
    println!("Positions: {:?}", positions);
    // let avg_x_pos = positions.map(|(x, _)| x).sum::<f64>() / data.len() as f64;
    // let avg_y_pos = positions.map(|(_, y)| y).sum::<f64>() / data.len() as f64;
    let (avg_x_pos, avg_y_pos) = positions
        .iter()
        .fold((0.0, 0.0), |(x, y), (x2, y2)| (x + x2, y + y2));
    let avg_pos = (avg_x_pos, avg_y_pos);
    // Get the angle of the average position
    let avg_angle = avg_pos.1.atan2(avg_pos.0);
    // Convert the angle to a value on the range
    let avg_value = (avg_angle / (2.0 * std::f64::consts::PI)) * range;
    // Get the error, which is the distance from the average position to the origin
    let error = (avg_pos.0.powi(2) + avg_pos.1.powi(2)).sqrt();
    (avg_value, error)
}

fn main() {
    let data = vec![0.0]; //, 1.0];
    let (avg, error) = circadian_average(4.0, data);
    println!("Average: {}, Error: {}", avg, error);
}
