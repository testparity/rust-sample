pub fn apply_discount(subtotal: f64, vip: bool) -> f64 {
    let rate = if vip { 0.20 } else { 0.10 };
    ((subtotal * (1.0 - rate)) * 100.0).round() / 100.0
}
