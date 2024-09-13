use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{prelude::FromPrimitive, Decimal, RoundingStrategy};

pub struct MarkupApplier {
    pub delta_bid: f64,
    pub delta_ask: f64,
    pub max_spread: Option<f64>,
    pub min_spread: Option<f64>,
    pub digits: u32,
    pub pip: f64,
    pub factor: f64,
}

impl Default for MarkupApplier {
    fn default() -> Self {
        MarkupApplier {
            delta_bid: 0.0,
            delta_ask: 0.0,
            max_spread: None,
            min_spread: None,
            digits: 0,
            pip: 0.0,
            factor: 0.0,
        }
    }
}

impl MarkupApplier {
    pub fn apply_markup(&self, price: f64, is_bid: bool) -> f64 {
        if is_bid {
            price + self.delta_bid
        } else {
            price + self.delta_ask
        }
    }

    pub fn apply_min_max_spread(&self, price: f64, is_bid: bool) -> f64 {
        if is_bid {
            price + self.delta_bid
        } else {
            price + self.delta_ask
        }
    }
}

pub fn get_max_spread(
    bid: f64,
    ask: f64,
    max_spread: f64,
    asset_factor: f64,
    pip: f64,
    digits: u32,
) -> Option<(f64, f64)> {
    let bid = Decimal::from_f64(bid).unwrap();
    let ask = Decimal::from_f64(ask).unwrap();
    let pip = Decimal::from_f64(pip).unwrap();
    let spread = ask - bid;
    let max_spread = Decimal::from_f64(max_spread).unwrap();

    if max_spread < spread {
        let is_odd = (spread.round_dp(digits).to_f64().unwrap() * asset_factor) as i32 % 2 == 0;
        let spread_diff = spread - max_spread;
        let spread_rounded = (spread_diff / Decimal::from_f64(2.0).unwrap())
            .round_dp_with_strategy(digits, RoundingStrategy::ToZero);

        if is_odd {
            return Some((
                (bid + spread_rounded).to_f64().unwrap(),
                (ask - spread_rounded).to_f64().unwrap(),
            ));
        } else {
            return Some((
                (bid + spread_rounded + pip).to_f64().unwrap(),
                (ask - spread_rounded).to_f64().unwrap(),
            ));
        }
    }

    None
}

pub fn get_min_spread(
    bid: f64,
    ask: f64,
    min_spread: f64,
    asset_factor: f64,
    pip: f64,
    digits: u32,
) -> Option<(f64, f64)> {
    let bid = Decimal::from_f64(bid).unwrap();
    let ask = Decimal::from_f64(ask).unwrap();
    let pip = Decimal::from_f64(pip).unwrap();
    let spread = ask - bid;
    let min_spread = Decimal::from_f64(min_spread).unwrap();

    if spread < min_spread {
        let is_odd = (spread.round_dp(digits).to_f64().unwrap() * asset_factor) as i32 % 2 == 0;
        let spread_diff = min_spread - spread;
        let spread_rounded = (spread_diff / Decimal::from_f64(2.0).unwrap())
            .round_dp_with_strategy(digits, RoundingStrategy::ToZero);

        if is_odd {
            return Some((
                (bid - spread_rounded).to_f64().unwrap(),
                (ask + spread_rounded).to_f64().unwrap(),
            ));
        } else {
            return Some((
                (bid - spread_rounded - pip).to_f64().unwrap(),
                (ask + spread_rounded).to_f64().unwrap(),
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_max_spread() {
        let (bid, ask) = get_max_spread(
            1.23414,
            1.23434,
            0.00010,
            10_f64.powi(5),
            1.0 / 10_f64.powi(5),
            5,
        )
        .unwrap();

        assert_eq!(format!("{:.5}", bid), "1.23419");
        assert_eq!(format!("{:.5}", ask), "1.23429");
    }

    #[test]
    fn test_apply_max_spread2() {
        let (bid, ask) = get_max_spread(
            1.23414,
            1.23434,
            0.00010,
            10_f64.powi(5),
            1.0 / 10_f64.powi(5),
            5,
        )
        .unwrap();

        assert_eq!(format!("{:.5}", bid), "1.23419");
        assert_eq!(format!("{:.5}", ask), "1.23429");
    }

    #[test]
    fn test_apply_min_spread() {
        let (bid, ask) = get_min_spread(
            1.23434,
            1.23435,
            0.00010,
            10_f64.powi(5),
            1.0 / 10_f64.powi(5),
            5,
        )
        .unwrap();

        assert_eq!(format!("{:.5}", bid), "1.23429");
        assert_eq!(format!("{:.5}", ask), "1.23439");
    }

    #[test]
    fn test_apply_min_spread2() {
        let (bid, ask) = get_min_spread(
            1.23434,
            1.23437,
            0.00010,
            10_f64.powi(5),
            1.0 / 10_f64.powi(5),
            5,
        )
        .unwrap();

        assert_eq!(format!("{:.5}", bid), "1.23430");
        assert_eq!(format!("{:.5}", ask), "1.23440");
    }

    #[test]
    fn test_max_zero() {

        let (bid, ask) = get_max_spread(
            1.23434,
            1.23436,
            0.0,
            10_f64.powi(5),
            1.0 / 10_f64.powi(5),
            5,
        ).unwrap();

        assert_eq!(format!("{:.5}", bid), "1.23435");
        assert_eq!(format!("{:.5}", ask), "1.23435");
    }
}
