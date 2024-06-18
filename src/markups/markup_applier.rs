pub struct MarkupApplier {
    pub delta_bid: f64,
    pub delta_ask: f64,
}

impl MarkupApplier {
    pub fn create_empty() -> MarkupApplier {
        MarkupApplier {
            delta_bid: 0.0,
            delta_ask: 0.0,
        }
    }

    pub fn apply_markup(&self, price: f64, is_bid: bool) -> f64 {
        if is_bid {
            price + self.delta_bid
        } else {
            price + self.delta_ask
        }
    }
}
