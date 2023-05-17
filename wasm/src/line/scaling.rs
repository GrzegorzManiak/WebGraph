pub fn autoscale_graph(height: f64, max_value: f64, min_value: f64) -> Vec<f64> {
    let scale_factor = height as f64 / (max_value - min_value);
    let mut scaled_values = Vec::new();

    for i in 0..height.ceil() as u32 {
        let scaled_value = max_value - (i as f64) / scale_factor;
        scaled_values.push(scaled_value);
    }

    scaled_values
}