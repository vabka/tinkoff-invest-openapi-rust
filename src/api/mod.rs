use std::error::Error;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use tonic::{
    codegen::InterceptedService,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, Endpoint},
};

use self::tinkoff_public_invest_api_contract_v1::{
    users_service_client::UsersServiceClient, MoneyValue, Quotation,
};

pub mod tinkoff_public_invest_api_contract_v1;

impl Into<Decimal> for MoneyValue {
    fn into(self) -> Decimal {
        Decimal::from(self.units) + Decimal::from(self.nano) / dec!(1_000_000)
    }
}

impl Into<Decimal> for Quotation {
    fn into(self) -> Decimal {
        Decimal::from(self.units) + Decimal::from(self.nano) / dec!(1_000_000)
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

impl TinkoffInvestClient {
    pub async fn connect(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let uri = "https://invest-public-api.tinkoff.ru:443";
        let interceptor = TinkoffSpecificHeadersInterceptor::new(token)?;
        let channel = Endpoint::new(uri)?.connect().await?;
        Ok(Self {
            channel,
            interceptor,
        })
    }

    pub fn get_users_service_client(
        &self,
    ) -> UsersServiceClient<InterceptedService<Channel, TinkoffSpecificHeadersInterceptor>> {
        UsersServiceClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }
}
