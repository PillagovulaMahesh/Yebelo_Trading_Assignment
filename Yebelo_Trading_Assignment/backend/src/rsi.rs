pub fn calculate_rsi(prices: Vec<f64>, period: usize) -> f64 {
    if prices.len() < period + 1 {
        return 0.0; // not enough data
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in (prices.len() - period)..(prices.len() - 1) {
        let change = prices[i + 1] - prices[i];
        if change > 0.0 {
            gains += change;
        } else {
            losses += -change;
        }
    }

    if losses == 0.0 {
        return 100.0;
    }

    let rs = gains / losses;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    rsi
}
