use std::ops::RangeInclusive;

use chrono::{Date, Utc, DateTime};
use tinkoff_invest_grpc::api;

use super::grpc_timestamp_to_chrono_timestamp;

#[repr(transparent)]
pub struct TradingDay(api::TradingDay);
impl From<api::TradingDay> for TradingDay {
    #[inline(always)]
    fn from(value: api::TradingDay) -> Self {
        Self(value)
    }
}

impl TradingDay {
    #[inline(always)]
    pub fn date(&self) -> Date<Utc> {
        let date = self.0.date.as_ref().unwrap();
        grpc_timestamp_to_chrono_timestamp(date).date()
    }

    #[inline(always)]
    pub fn is_trading_day(&self) -> bool {
        self.0.is_trading_day
    }
    
    #[inline(always)]
    pub fn trading_time(&self) -> Option<RangeInclusive<DateTime<Utc>>> {
        let start = self.0.start_time.as_ref()?;
        let end = self.0.end_time.as_ref()?;
        let start = grpc_timestamp_to_chrono_timestamp(start);
        let end = grpc_timestamp_to_chrono_timestamp(end);
        Some(start..=end)
    }

    #[inline(always)]
    pub fn opening_auction_start_time(&self) -> Option<DateTime<Utc>> {
        self.0
            .opening_auction_start_time
            .as_ref()
            .map(grpc_timestamp_to_chrono_timestamp)
    }

    #[inline(always)]
    pub fn closing_auction_end_time(&self) -> Option<DateTime<Utc>> {
        self.0
            .closing_auction_end_time
            .as_ref()
            .map(grpc_timestamp_to_chrono_timestamp)
    }

    #[inline(always)]
    pub fn evening_opening_auction_start_time(&self) -> Option<DateTime<Utc>> {
        self.0
            .evening_opening_auction_start_time
            .as_ref()
            .map(grpc_timestamp_to_chrono_timestamp)
    }

    #[inline(always)]
    pub fn evening_trading_time(&self) -> Option<RangeInclusive<DateTime<Utc>>> {
        let start = grpc_timestamp_to_chrono_timestamp(self.0.evening_start_time.as_ref()?);
        let end = grpc_timestamp_to_chrono_timestamp(self.0.evening_end_time.as_ref()?);
        Some(start..=end)
    }

    #[inline(always)]
    pub fn clearing_time(&self) -> Option<RangeInclusive<DateTime<Utc>>> {
        let start = grpc_timestamp_to_chrono_timestamp(self.0.clearing_start_time.as_ref()?);
        let end = grpc_timestamp_to_chrono_timestamp(self.0.clearing_end_time.as_ref()?);
        Some(start..=end)
    }

    #[inline(always)]
    pub fn premarket_time(&self) -> Option<RangeInclusive<DateTime<Utc>>> {
        let start = grpc_timestamp_to_chrono_timestamp(self.0.premarket_start_time.as_ref()?);
        let end = grpc_timestamp_to_chrono_timestamp(self.0.premarket_end_time.as_ref()?);
        Some(start..=end)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TradingSchedule(api::TradingSchedule);
impl From<api::TradingSchedule> for TradingSchedule {
    #[inline(always)]
    fn from(value: api::TradingSchedule) -> Self {
        Self(value)
    }
}
impl TradingSchedule {
    #[inline(always)]
    pub fn exchange(&self) -> &str {
        &self.0.exchange
    }

    #[inline(always)]
    pub fn days(&self) -> &[TradingDay] {
        unsafe { std::mem::transmute(self.0.days.as_slice()) }
    }
}