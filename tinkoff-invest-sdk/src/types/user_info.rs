use tinkoff_invest_grpc::api;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserInfo(api::GetInfoResponse);
impl From<api::GetInfoResponse> for UserInfo {
    #[inline(always)]
    fn from(r: api::GetInfoResponse) -> UserInfo {
        UserInfo(r)
    }
}
impl UserInfo {
    #[inline(always)]
    pub fn is_premium(&self) -> bool {
        self.0.prem_status
    }

    #[inline(always)]
    pub fn is_qualified(&self) -> bool {
        self.0.qual_status
    }

    #[inline(always)]
    pub fn qualified_for_work_with(&self) -> &[String] {
        &self.0.qualified_for_work_with.as_slice()
    }

    #[inline(always)]
    pub fn tariff(&self) -> &str {
        &self.0.tariff
    }
}

#[derive(Debug, Clone)]
pub struct UnaryLimit(api::UnaryLimit);
impl From<api::UnaryLimit> for UnaryLimit {
    #[inline(always)]
    fn from(inner: api::UnaryLimit) -> Self {
        UnaryLimit(inner)
    }
}
impl UnaryLimit {
    /// Количество unary-запросов в минуту
    #[inline(always)]
    pub fn limit_per_minute(&self) -> i32 {
        self.0.limit_per_minute
    }

    /// Названия методов
    #[inline(always)]
    pub fn methods(&self) -> &[String] {
        &self.0.methods
    }
}

#[derive(Debug, Clone)]
pub struct StreamLimit(api::StreamLimit);
impl From<api::StreamLimit> for StreamLimit {
    #[inline(always)]
    fn from(inner: api::StreamLimit) -> Self {
        StreamLimit(inner)
    }
}
impl StreamLimit {
    /// Максимальное количество stream-соединений
    #[inline(always)]
    pub fn limit(&self) -> i32 {
        self.0.limit
    }

    /// Названия stream-методов
    #[inline(always)]
    pub fn streams(&self) -> &[String] {
        self.0.streams.as_slice()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct UserTariff(api::GetUserTariffResponse);
impl From<api::GetUserTariffResponse> for UserTariff {
    #[inline(always)]
    fn from(response: api::GetUserTariffResponse) -> Self {
        UserTariff(response)
    }
}
impl UserTariff {
    #[inline(always)]
    pub fn unary_limits(&self) -> &[UnaryLimit] {
        let borrowed = self.0.unary_limits.as_slice();
        // Безопасно, так как UnaryLimit должен быть такого же размера и структуры, как и api::UnaryLimit
        // borrow-checker ломаться не должен из-за этого
        // note: было бы неплохо добавить сюда статическую проверку
        // note: Возможно, можно как-то это сделать без unsafe
        unsafe { ::std::mem::transmute(borrowed) }
    }

    #[inline(always)]
    pub fn stream_limits(&self) -> &[StreamLimit] {
        let borrowed = self.0.stream_limits.as_slice();
        // Безопасно, так как StreamLimit должен быть такого же размера и структуры, как и api::StreamLimit
        // borrow-checker ломаться не должен из-за этого
        // note: было бы неплохо добавить сюда статическую проверку
        // note: Возможно, можно как-то это сделать без unsafe
        unsafe { ::std::mem::transmute(borrowed) }
    }
}