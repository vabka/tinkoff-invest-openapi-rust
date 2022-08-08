use chrono::{Date, Utc};
use tinkoff_invest_grpc::{api, decimal::rust_decimal::Decimal};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Currency(pub String);
#[derive(Debug, Clone, Copy)]
pub struct Short {
    /// Коэффициент ставки риска короткой позиции по инструменту.
    kshort: Decimal,
    /// Ставка риска минимальной маржи в шорт.
    dshort: Decimal,
    /// Ставка риска начальной маржи в шорт.
    dshort_min: Decimal,
}

#[derive(Debug, Clone, Copy)]
pub struct Long {
    /// Коэффициент ставки риска длинной позиции по инструменту.
    klong: Decimal,
    /// Ставка риска минимальной маржи в лонг.
    dlong: Decimal,
    /// Ставка риска начальной маржи в лонг.
    dlong_min: Decimal,
}

#[derive(Debug, Clone)]
pub struct MoneyValue {
    currency: Currency,
    amount: Decimal,
}
#[derive(Debug, Clone)]
pub struct CountryOfRisk {
    code: String,
    name: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён
    Unspecified = 0,
    /// Недоступен для торгов
    NotAvailableForTrading = 1,
    /// Период открытия торгов
    OpeningPeriod = 2,
    /// Период закрытия торгов
    ClosingPeriod = 3,
    /// Перерыв в торговле
    BreakInTrading = 4,
    /// Нормальная торговля
    NormalTrading = 5,
    /// Аукцион закрытия
    ClosingAuction = 6,
    /// Аукцион крупных пакетов
    DarkPoolAuction = 7,
    /// Дискретный аукцион
    DiscreteAuction = 8,
    /// Аукцион открытия
    OpeningAuctionPeriod = 9,
    /// Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice = 10,
    /// Сессия назначена
    SessionAssigned = 11,
    /// Сессия закрыта
    SessionClose = 12,
    /// Сессия открыта
    SessionOpen = 13,
    /// Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading = 14,
    /// Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading = 15,
    /// Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading = 16,
}

#[derive(Debug, Clone, Copy)]
pub enum BondIssueKind {
    Unknown,
    Documentary,
    NonDocumentary,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RealExchange {
    Unspecified = 0,
    Moex = 1,
    Rts = 2,
    Otc = 3,
}
#[derive(Debug, Clone)]
pub struct Bond {
    /// Figi-идентификатор инструмента.
    figi: String,
    /// Тикер инструмента.
    ticker: String,
    /// Класс-код (секция торгов).
    class_code: String,
    /// Isin-идентификатор инструмента.
    isin: String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*.
    lot: i32,
    /// Валюта расчётов.
    currency: Currency,
    short: Option<Short>,
    long: Option<Long>,
    name: String,
    exchange: String,
    coupon_quantity_per_year: u32,
    maturity_date: Date<Utc>,
    nominal: Option<MoneyValue>,
    state_reg_date: Date<Utc>,
    placement_date: Date<Utc>,
    placement_price: Option<MoneyValue>,
    aci_value: Option<MoneyValue>,
    country_of_risk: CountryOfRisk,
    sector: String,
    issue_kind: BondIssueKind,
    issue_size: u64,
    issue_size_plan: u64,
    trading_status: SecurityTradingStatus,
    is_otc: bool,
    purchase_available: bool,
    sell_available: bool,
    with_floating_coupon: bool,
    is_perpetual: bool,
    with_amortization: bool,
    min_price_increment: Option<Decimal>,
    api_trade_available: bool,
    uid: String,
    real_exchange: RealExchange,
    position_uid: String,
    available_for_iis: bool,
    first_minute_candle_date: Date<Utc>,
    first_day_candle_date: Date<Utc>,
}

impl From<api::Bond> for Bond {
    fn from(bond: api::Bond) -> Self {
        todo!()
    }
}
