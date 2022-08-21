use tinkoff_invest_grpc::{api, decimal::rust_decimal::Decimal};

use super::MoneyValue;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MarginAttributes(api::GetMarginAttributesResponse);
impl From<api::GetMarginAttributesResponse> for MarginAttributes {
    #[inline(always)]
    fn from(r: api::GetMarginAttributesResponse) -> MarginAttributes {
        MarginAttributes(r)
    }
}

impl MarginAttributes {
    /// Ликвидная стоимость портфеля.
    #[inline(always)]
    pub fn liquid_portfolio(&self) -> Option<MoneyValue> {
        self.0.liquid_portfolio.clone().map(MoneyValue::from)
    }

    /// Начальная маржа — начальное обеспечение для совершения новой сделки.
    #[inline(always)]
    pub fn starting_margin(&self) -> Option<MoneyValue> {
        self.0.starting_margin.clone().map(MoneyValue::from)
    }

    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли.
    #[inline(always)]
    pub fn minimal_margin(&self) -> Option<MoneyValue> {
        self.0.minimal_margin.clone().map(MoneyValue::from)
    }

    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[inline(always)]
    pub fn funds_sufficiency_level(&self) -> Option<Decimal> {
        self.0.funds_sufficiency_level.clone().map(Decimal::from)
    }

    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[inline(always)]
    pub fn amount_of_missing_funds(&self) -> Option<MoneyValue> {
        self.0.amount_of_missing_funds.clone().map(MoneyValue::from)
    }
}