use std::sync::Arc;

use my_nosql_contracts::{
    MarkupProfileNoSqlEntity, TradingGroupNoSqlEntity, TradingInstrumentNoSqlEntity,
};
use service_sdk::async_trait::async_trait;

pub struct MarkupApplier {
    pub bid: f64,
    pub ask: f64,
}

impl MarkupApplier {
    pub fn apply(&self, bid: f64, ask: f64) -> (f64, f64) {
        (bid + self.bid, ask + self.ask)
    }
}

#[async_trait]
pub trait MarkupUtils {
    async fn get_instrument(
        &self,
        instrument_id: &str,
    ) -> Option<Arc<TradingInstrumentNoSqlEntity>>;
    async fn get_trading_group(&self, group_id: &str) -> Option<Arc<TradingGroupNoSqlEntity>>;

    async fn get_markup_profile(&self, profile_id: &str) -> Option<Arc<MarkupProfileNoSqlEntity>>;

    async fn get_markup_applier(
        &self,
        instrument_id: &str,
        trading_group_id: &str,
    ) -> Option<MarkupApplier> {
        let trading_group = self.get_trading_group(&trading_group_id).await?;

        let instrument = self.get_instrument(trading_group_id).await?;

        let markup_profile_id = trading_group.markup_profile_id.as_ref()?;

        let profile = self.get_markup_profile(markup_profile_id).await?;

        if profile.disabled {
            return None;
        }

        let markup_profile = profile.instruments.get(instrument_id)?;

        let multiplier = 1.0 / i64::pow(10, instrument.digits as u32) as f64;

        let result = MarkupApplier {
            bid: markup_profile.markup_bid as f64 * multiplier,
            ask: markup_profile.markup_ask as f64 * multiplier,
        };

        Some(result)
    }
}
