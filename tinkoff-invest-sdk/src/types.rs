use chrono::{Date, DateTime, TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use tinkoff_invest_grpc::{api, decimal::rust_decimal::Decimal};

#[derive(Debug, Clone)]
pub struct MarginAttributes(api::GetMarginAttributesResponse);
impl From<api::GetMarginAttributesResponse> for MarginAttributes {
    fn from(r: api::GetMarginAttributesResponse) -> MarginAttributes {
        MarginAttributes(r)
    }
}
impl MarginAttributes {
    /// Ликвидная стоимость портфеля.
    #[inline]
    pub fn liquid_portfolio(&self) -> Option<MoneyValue> {
        self.0.liquid_portfolio.clone().map(MoneyValue::from)
    }

    /// Начальная маржа — начальное обеспечение для совершения новой сделки.
    #[inline]
    pub fn starting_margin(&self) -> Option<MoneyValue> {
        self.0.starting_margin.clone().map(MoneyValue::from)
    }

    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли.
    #[inline]
    pub fn minimal_margin(&self) -> Option<MoneyValue> {
        self.0.minimal_margin.clone().map(MoneyValue::from)
    }
    
    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[inline]
    pub fn funds_sufficiency_level(&self) -> Option<Decimal> {
        self.0.funds_sufficiency_level.clone().map(Decimal::from)
    }

    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[inline]
    pub fn amount_of_missing_funds(&self) -> Option<MoneyValue> {
        self.0.amount_of_missing_funds.clone().map(MoneyValue::from)
    }
}
#[derive(Debug, Clone)]
pub struct Info(api::GetInfoResponse);
impl From<api::GetInfoResponse> for Info {
    #[inline]
    fn from(r: api::GetInfoResponse) -> Info {
        Info(r)
    }
}

impl Info {
    #[inline]
    pub fn is_premium(&self) -> bool {
        self.0.prem_status
    }

    #[inline]
    pub fn is_qualified(&self) -> bool {
        self.0.qual_status
    }

    #[inline]
    pub fn qualified_for_work_with(&self) -> &[String] {
        &self.0.qualified_for_work_with.as_slice()
    }

    #[inline]
    pub fn tariff(&self) -> &str {
        &self.0.tariff
    }
}
#[derive(Debug, Clone)]
pub struct UserTariff(api::GetUserTariffResponse);
impl From<api::GetUserTariffResponse> for UserTariff {
    fn from(response: api::GetUserTariffResponse) -> Self {
        UserTariff(response)
    }
}
#[derive(Debug, Clone)]
pub struct UnaryLimit(api::UnaryLimit);
impl From<api::UnaryLimit> for UnaryLimit {
    #[inline]
    fn from(inner: api::UnaryLimit) -> Self {
        UnaryLimit(inner)
    }
}
impl UnaryLimit {
    /// Количество unary-запросов в минуту
    #[inline]
    pub fn limit_per_minute(&self) -> i32 {
        self.0.limit_per_minute
    }
    /// Названия методов
    #[inline]
    pub fn methods(&self) -> &[String] {
        &self.0.methods
    }
}
#[derive(Debug, Clone)]
pub struct StreamLimit(api::StreamLimit);
impl From<api::StreamLimit> for StreamLimit {
    #[inline]
    fn from(inner: api::StreamLimit) -> Self {
        StreamLimit(inner)
    }
}
impl StreamLimit {
    /// Максимальное количество stream-соединений
    #[inline]
    pub fn limit(&self) -> i32 {
        self.0.limit
    }

    /// Названия stream-методов
    #[inline]
    pub fn streams(&self) -> &[String] {
        self.0.streams.as_slice()
    }
}
impl UserTariff {
    #[inline]
    pub fn unary_limits(&self) -> &[UnaryLimit] {
        let borrowed = self.0.unary_limits.as_slice();
        // Безопасно, так как UnaryLimit должен быть такого же размера и структуры, как и api::UnaryLimit
        // borrow-checker ломаться не должен из-за этого
        // note: было бы неплохо добавить сюда статическую проверк
        unsafe { ::std::mem::transmute(borrowed) }
    }

    #[inline]
    pub fn stream_limits(&self) -> &[StreamLimit] {
        let borrowed = self.0.stream_limits.as_slice();
        // Безопасно, так как StreamLimit должен быть такого же размера и структуры, как и api::StreamLimit
        // borrow-checker ломаться не должен из-за этого
        // note: было бы неплохо добавить сюда статическую проверк
        unsafe { ::std::mem::transmute(borrowed) }
    }
}
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MoneyValue {
    pub currency: String,
    pub amount: Decimal,
}

impl From<api::MoneyValue> for MoneyValue {
    fn from(value: api::MoneyValue) -> Self {
        let (currency, amount) = value.into();
        Self { currency, amount }
    }
}
#[derive(Debug, Clone)]
pub struct CountryOfRisk {
    code: String,
    name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён
    Unspecified,
    /// Недоступен для торгов
    NotAvailableForTrading,
    /// Период открытия торгов
    OpeningPeriod,
    /// Период закрытия торгов
    ClosingPeriod,
    /// Перерыв в торговле
    BreakInTrading,
    /// Нормальная торговля
    NormalTrading,
    /// Аукцион закрытия
    ClosingAuction,
    /// Аукцион крупных пакетов
    DarkPoolAuction,
    /// Дискретный аукцион
    DiscreteAuction,
    /// Аукцион открытия
    OpeningAuctionPeriod,
    /// Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice,
    /// Сессия назначена
    SessionAssigned,
    /// Сессия закрыта
    SessionClose,
    /// Сессия открыта
    SessionOpen,
    /// Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading,
    /// Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading,
    /// Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading,
}

impl From<api::SecurityTradingStatus> for SecurityTradingStatus {
    fn from(value: api::SecurityTradingStatus) -> Self {
        match value {
            api::SecurityTradingStatus::Unspecified => Self::Unspecified,
            api::SecurityTradingStatus::NotAvailableForTrading => Self::NotAvailableForTrading,
            api::SecurityTradingStatus::OpeningPeriod => Self::OpeningPeriod,
            api::SecurityTradingStatus::ClosingPeriod => Self::ClosingPeriod,
            api::SecurityTradingStatus::BreakInTrading => Self::BreakInTrading,
            api::SecurityTradingStatus::NormalTrading => Self::NormalTrading,
            api::SecurityTradingStatus::ClosingAuction => Self::ClosingAuction,
            api::SecurityTradingStatus::DarkPoolAuction => Self::DarkPoolAuction,
            api::SecurityTradingStatus::DiscreteAuction => Self::DiscreteAuction,
            api::SecurityTradingStatus::OpeningAuctionPeriod => Self::OpeningAuctionPeriod,
            api::SecurityTradingStatus::TradingAtClosingAuctionPrice => {
                Self::TradingAtClosingAuctionPrice
            }
            api::SecurityTradingStatus::SessionAssigned => Self::SessionAssigned,
            api::SecurityTradingStatus::SessionClose => Self::SessionClose,
            api::SecurityTradingStatus::SessionOpen => Self::SessionOpen,
            api::SecurityTradingStatus::DealerNormalTrading => Self::DealerNormalTrading,
            api::SecurityTradingStatus::DealerBreakInTrading => Self::DealerBreakInTrading,
            api::SecurityTradingStatus::DealerNotAvailableForTrading => {
                Self::DealerNotAvailableForTrading
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BondIssueKind {
    Unknown,
    Documentary,
    NonDocumentary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RealExchange {
    Unspecified,
    Moex,
    Rts,
    Otc,
}

impl From<api::RealExchange> for RealExchange {
    fn from(value: api::RealExchange) -> Self {
        match value {
            api::RealExchange::Unspecified => Self::Unspecified,
            api::RealExchange::Moex => Self::Moex,
            api::RealExchange::Rts => Self::Rts,
            api::RealExchange::Otc => Self::Otc,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Bond(api::Bond);

impl From<api::Bond> for Bond {
    fn from(bond: api::Bond) -> Self {
        Self(bond)
    }
}

impl Bond {
    #[inline]
    pub fn figi(&self) -> &str {
        &self.0.figi
    }

    #[inline]
    pub fn ticker(&self) -> &str {
        &self.0.ticker
    }

    #[inline]
    pub fn class_code(&self) -> &str {
        &self.0.class_code
    }

    #[inline]
    pub fn isin(&self) -> &str {
        &self.0.isin
    }

    #[inline]
    pub fn lot(&self) -> i32 {
        self.0.lot
    }

    #[inline]
    pub fn currency(&self) -> &str {
        &self.0.currency
    }

    #[inline]
    pub fn short(&self) -> Option<Short> {
        if self.0.short_enabled_flag {
            // TODO extra allocation
            Some(Short {
                kshort: self.0.kshort.clone().map(Into::into)?,
                dshort: self.0.dshort.clone().map(Into::into)?,
                dshort_min: self.0.dshort_min.clone().map(Into::into)?,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn long(&self) -> Option<Long> {
        // TODO extra allocation
        Some(Long {
            klong: self.0.klong.clone().map(Into::into)?,
            dlong: self.0.dlong.clone().map(Into::into)?,
            dlong_min: self.0.dlong_min.clone().map(Into::into)?,
        })
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    #[inline]
    pub fn coupon_quantity_per_year(&self) -> u32 {
        self.0.coupon_quantity_per_year as u32
    }

    #[inline]
    pub fn maturity_date(&self) -> Option<Date<Utc>> {
        // TODO extra allocation
        self.0
            .maturity_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
            .map(|d| d.date())
    }

    #[inline]
    pub fn nominal(&self) -> Option<MoneyValue> {
        // TODO extra allocation
        self.0.nominal.clone().map(Into::into)
    }

    #[inline]
    pub fn state_reg_date(&self) -> Option<Date<Utc>> {
        // TODO extra allocation
        self.0
            .state_reg_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
            .map(|d| d.date())
    }

    #[inline]
    pub fn placement_date(&self) -> Option<Date<Utc>> {
        // TODO extra allocation
        self.0
            .placement_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
            .map(|d| d.date())
    }

    #[inline]
    pub fn placement_price(&self) -> Option<MoneyValue> {
        // TODO extra allocation
        self.0.placement_price.clone().map(Into::into)
    }

    #[inline]
    pub fn aci_value(&self) -> Option<MoneyValue> {
        // TODO extra allocation
        self.0.aci_value.clone().map(Into::into)
    }

    #[inline]
    pub fn country_of_risk(&self) -> CountryOfRisk {
        // TODO extra allocation
        CountryOfRisk {
            code: self.0.country_of_risk.clone(),
            name: self.0.country_of_risk_name.clone(),
        }
    }

    #[inline]
    pub fn sector(&self) -> &str {
        &self.0.sector
    }

    #[inline]
    pub fn issue_kind(&self) -> BondIssueKind {
        match self.0.issue_kind.as_str() {
            "documentary" => BondIssueKind::Documentary,
            "non_documentary" => BondIssueKind::NonDocumentary,
            _ => BondIssueKind::Unknown,
        }
    }

    #[inline]
    pub fn issue_size(&self) -> u64 {
        self.0.issue_size as u64
    }

    #[inline]
    pub fn issue_size_plan(&self) -> u64 {
        self.0.issue_size_plan as u64
    }

    #[inline]
    pub fn trading_status(&self) -> SecurityTradingStatus {
        self.0.trading_status().into()
    }
    #[inline]
    pub fn is_otc(&self) -> bool {
        self.0.otc_flag
    }

    #[inline]
    pub fn purchase_available(&self) -> bool {
        self.0.buy_available_flag
    }

    #[inline]
    pub fn sell_available_flag(&self) -> bool {
        self.0.sell_available_flag
    }

    #[inline]
    pub fn with_floating_coupon(&self) -> bool {
        self.0.floating_coupon_flag
    }

    #[inline]
    pub fn is_perpetual(&self) -> bool {
        self.0.perpetual_flag
    }

    #[inline]
    pub fn with_amortization(&self) -> bool {
        self.0.amortization_flag
    }

    #[inline]
    pub fn min_price_increment(&self) -> Option<Decimal> {
        // TODO allocation
        self.0.min_price_increment.clone().map(Into::into)
    }

    #[inline]
    pub fn api_trade_available(&self) -> bool {
        self.0.api_trade_available_flag
    }

    #[inline]
    pub fn uid(&self) -> &str {
        &self.0.uid
    }

    #[inline]
    pub fn real_exchange(&self) -> RealExchange {
        self.0.real_exchange().into()
    }

    #[inline]
    pub fn position_uid(&self) -> &str {
        &self.0.position_uid
    }

    #[inline]
    pub fn available_for_iis(&self) -> bool {
        self.0.for_iis_flag
    }

    #[inline]
    pub fn first_minute_candle_date(&self) -> Option<DateTime<Utc>> {
        // TODO allocation
        self.0
            .first_1day_candle_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
    }

    #[inline]
    pub fn first_day_candle_date(&self) -> Option<DateTime<Utc>> {
        // TODO allocation
        self.0
            .first_1day_candle_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
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
pub enum AccountAccessLevel {
    /// Уровень доступа не определён.
    Unspecified,
    /// Полный доступ к счёту.
    FullAccess,
    /// Доступ с уровнем прав "только чтение".
    ReadOnly,
    /// Доступ отсутствует.
    NoAccess,
}

#[derive(Debug, Clone)]
pub struct Account(api::Account);
impl Account {
    pub fn id(&self) -> &str {
        &self.0.id
    }

    pub fn account_type(&self) -> AccountType {
        match self.0.r#type {
            1 => AccountType::Tinkoff,
            2 => AccountType::TinkoffIis,
            3 => AccountType::InvestBox,
            _ => AccountType::Unspecified,
        }
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn status(&self) -> AccountStatus {
        match self.0.status() {
            api::AccountStatus::Unspecified => AccountStatus::Unspecified,
            api::AccountStatus::New => AccountStatus::New,
            api::AccountStatus::Open => AccountStatus::Open,
            api::AccountStatus::Closed => AccountStatus::Closed,
        }
    }

    pub fn opened_date(&self) -> Option<Date<Utc>> {
        self.0
            .opened_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
            .map(|t| t.date())
    }

    pub fn closed_date(&self) -> Option<Date<Utc>> {
        self.0
            .closed_date
            .clone()
            .and_then(grpc_timestamp_to_chrono_timestamp)
            .map(|t| t.date())
    }

    pub fn access_level(&self) -> AccountAccessLevel {
        match self.0.access_level() {
            api::AccessLevel::AccountAccessLevelUnspecified => AccountAccessLevel::Unspecified,
            api::AccessLevel::AccountAccessLevelFullAccess => AccountAccessLevel::FullAccess,
            api::AccessLevel::AccountAccessLevelReadOnly => AccountAccessLevel::ReadOnly,
            api::AccessLevel::AccountAccessLevelNoAccess => AccountAccessLevel::NoAccess,
        }
    }
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
        Self(account)
    }
}

fn grpc_timestamp_to_chrono_timestamp(t: prost_types::Timestamp) -> Option<DateTime<Utc>> {
    SystemTime::try_from(t).map(system_time_to_date_time).ok()
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
