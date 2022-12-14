mod generated;
pub use generated::tinkoff_invest_v1 as api;
use generated::tinkoff_invest_v1::{
    instruments_service_client::InstrumentsServiceClient,
    market_data_service_client::MarketDataServiceClient,
    market_data_stream_service_client::MarketDataStreamServiceClient,
    operations_service_client::OperationsServiceClient,
    operations_stream_service_client::OperationsStreamServiceClient,
    orders_service_client::OrdersServiceClient,
    orders_stream_service_client::OrdersStreamServiceClient,
    sandbox_service_client::SandboxServiceClient,
    stop_orders_service_client::StopOrdersServiceClient, users_service_client::UsersServiceClient,
    MoneyValue, Quotation,
};

pub mod decimal {
    pub use rust_decimal;
    pub use rust_decimal_macros::dec;
}
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::error::Error;
pub use tonic;

use tonic::{
    codegen::InterceptedService,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, Endpoint},
};

impl From<Quotation> for Decimal {
    fn from(value: Quotation) -> Self {
        Decimal::from(value.units) + Decimal::from(value.nano) / dec!(1_000_000_000)
    }
}

impl From<MoneyValue> for (String, Decimal) {
    fn from(value: MoneyValue) -> Self {
        (
            value.currency,
            Decimal::from(value.units) + Decimal::from(value.nano) / dec!(1_000_000_000)
        )
    }
}

#[cfg(test)]
mod decimal_tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn convert_zero() {
        let value = super::api::Quotation {
            units: 0i64,
            nano: 0,
        };
        let decimal: Decimal = value.into();
        assert_eq!(dec!(0), decimal);
    }

    #[test]
    fn convert_negative() {
        let value = super::api::Quotation {
            units: -5i64,
            nano: -990000000,
        };
        let result: Decimal = value.into();
        assert_eq!(dec!(-5.99), result);
    }
    #[test]
    fn convert_positive() {
        let value = super::api::Quotation {
            units: 5i64,
            nano: 990000000,
        };
        let result: Decimal = value.into();
        assert_eq!(dec!(5.99), result);
    }
}

#[derive(Clone)]
pub struct TinkoffSpecificHeadersInterceptor {
    authorization_header_value: MetadataValue<Ascii>,
    x_app_name_header_value: MetadataValue<Ascii>,
}
impl TinkoffSpecificHeadersInterceptor {
    fn new(token: &str) -> Result<Self, Box<dyn Error>> {
        let authorization_header_value: MetadataValue<Ascii> =
            format!("Bearer {token}").try_into()?;
        let x_app_name_header_value = MetadataValue::<Ascii>::from_static("rust_sdk");
        Ok(Self {
            authorization_header_value,
            x_app_name_header_value,
        })
    }
}

impl Interceptor for TinkoffSpecificHeadersInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let metadata = request.metadata_mut();
        metadata.insert("authorization", self.authorization_header_value.clone());
        metadata.insert("x-app-name", self.x_app_name_header_value.clone());
        Ok(request)
    }
}

pub struct TinkoffInvestClient {
    channel: Channel,
    interceptor: TinkoffSpecificHeadersInterceptor,
}

pub type Inner = InterceptedService<Channel, TinkoffSpecificHeadersInterceptor>;
impl TinkoffInvestClient {
    pub async fn connect(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let uri = "https://invest-public-api.tinkoff.ru:443";
        let interceptor = TinkoffSpecificHeadersInterceptor::new(token)?;
        let channel = Endpoint::from_static(uri).connect().await?;
        Ok(Self {
            channel,
            interceptor,
        })
    }

    pub fn users(&self) -> UsersServiceClient<Inner> {
        UsersServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn instruments(&self) -> InstrumentsServiceClient<Inner> {
        InstrumentsServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn market_data(&self) -> MarketDataServiceClient<Inner> {
        MarketDataServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn market_data_stream(&self) -> MarketDataStreamServiceClient<Inner> {
        MarketDataStreamServiceClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn operations(&self) -> OperationsServiceClient<Inner> {
        OperationsServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }
    pub fn operations_stream(&self) -> OperationsStreamServiceClient<Inner> {
        OperationsStreamServiceClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        )
    }

    pub fn orders(&self) -> OrdersServiceClient<Inner> {
        OrdersServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn orders_stream(&self) -> OrdersStreamServiceClient<Inner> {
        OrdersStreamServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn sandbox(&self) -> SandboxServiceClient<Inner> {
        SandboxServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    pub fn stop_orders(&self) -> StopOrdersServiceClient<Inner> {
        StopOrdersServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }
}
