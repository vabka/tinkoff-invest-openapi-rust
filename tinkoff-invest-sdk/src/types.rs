use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Date, DateTime, TimeZone, Utc};
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccountType {
    Unspecified,
    Tinkoff,
    TinkoffIis,
    InvestBox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AccessLevel {
    /// Уровень доступа не определён.
    Unspecified = 0,
    /// Полный доступ к счёту.
    FullAccess = 1,
    /// Доступ с уровнем прав "только чтение".
    ReadOnly = 2,
    /// Доступ отсутствует.
    NoAccess = 3,
}

#[derive(Debug, Clone)]
pub struct Account {
    id: String,
    account_type: AccountType,
    name: String,
    status: AccountStatus,
    opened_date: Option<Date<Utc>>,
    closed_date: Option<Date<Utc>>,
    access_level: AccessLevel,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AccountStatus {
    Unspecified,
    New,
    Open,
    Closed,
}
impl From<api::Account> for Account {
    fn from(account: api::Account) -> Self {
        Self {
            id: account.id,
            account_type: match account.r#type {
                1 => AccountType::Tinkoff,
                2 => AccountType::TinkoffIis,
                3 => AccountType::InvestBox,
                _ => AccountType::Unspecified,
            },
            name: account.name,
            status: match account.status {
                1 => AccountStatus::New,
                2 => AccountStatus::Open,
                3 => AccountStatus::Closed,
                _ => AccountStatus::Unspecified,
            },
            opened_date: account
                .opened_date
                .map(SystemTime::try_from)
                .map(Result::unwrap)
                .map(system_time_to_date_time)
                .map(|dt| dt.date()),
            closed_date: account
                .closed_date
                .map(SystemTime::try_from)
                .map(Result::unwrap)
                .map(system_time_to_date_time)
                .map(|dt| dt.date()),

            access_level: match account.access_level {
                1 => AccessLevel::FullAccess,
                2 => AccessLevel::ReadOnly,
                3 => AccessLevel::NoAccess,
                _ => AccessLevel::Unspecified,
            },
        }
    }
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Utc.timestamp(sec, nsec)
}
