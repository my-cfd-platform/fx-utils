use std::sync::Arc;

use my_nosql_contracts::MarkupProfileNoSqlEntity;

use super::*;

#[derive(Debug)]
pub enum MarkupCalculatorError {
    InstrumentNotFound,
}

#[async_trait::async_trait]
pub trait IMarkupCalculator {
    async fn get_markup_profile_id(&self, group_id: &str) -> Option<String>;

    async fn get_markup_profile(
        &self,
        markup_profile_id: &str,
    ) -> Option<Arc<MarkupProfileNoSqlEntity>>;

    async fn get_instrument_digits(&self, instrument_id: &str) -> Option<u32>;

    async fn get_markup_applier(
        &self,
        group_id: &str,
        instrument_id: &str,
    ) -> Result<MarkupApplier, MarkupCalculatorError> {
        let profile_id = self.get_markup_profile_id(group_id).await;

        if profile_id.is_none() {
            return Ok(MarkupApplier::create_empty());
        }

        let profile_id = profile_id.unwrap();

        let markup_profile = self.get_markup_profile(&profile_id).await;

        if markup_profile.is_none() {
            return Ok(MarkupApplier::create_empty());
        }

        let markup_profile = markup_profile.unwrap();

        if markup_profile.disabled {
            return Ok(MarkupApplier::create_empty());
        }

        let markup_by_instrument = markup_profile.instruments.get(instrument_id);

        if markup_by_instrument.is_none() {
            return Ok(MarkupApplier::create_empty());
        }

        let markup_by_instrument = markup_by_instrument.unwrap();

        let instrument_digits = self.get_instrument_digits(instrument_id).await;

        if instrument_digits.is_none() {
            return Err(MarkupCalculatorError::InstrumentNotFound);
        }

        let multiplier = 1.0 / i64::pow(10, instrument_digits.unwrap()) as f64;

        Ok(MarkupApplier {
            delta_bid: multiplier * markup_by_instrument.markup_bid as f64,
            delta_ask: multiplier * markup_by_instrument.markup_ask as f64,
        }
        .into())
    }
}
