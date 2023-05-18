pub fn calculate_tick_range(range: (f64, f64), tick_count: u32) -> f64 {
    let (lower_bound, upper_bound) = range;
    let unrounded_tick_size = (upper_bound - lower_bound) / (tick_count - 1) as f64;
    let x = (unrounded_tick_size.log10().ceil() - 1.0).floor();
    let pow10x = 10.0_f64.powf(x);
    let rounded_tick_range = (unrounded_tick_size / pow10x).ceil() * pow10x;
    rounded_tick_range
}
