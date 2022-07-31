mod contracts;
use std::{error::Error, fmt::Display};

use contracts::tinkoff_public_invest_api_contract_v1::users_service_client::*;
use contracts::tinkoff_public_invest_api_contract_v1::*;
use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::Endpoint,
};

#[derive(Clone)]
struct TinkoffSpecificHeadersInterceptor {
    authorization_header_value: MetadataValue<Ascii>,
    x_app_name_header_value: MetadataValue<Ascii>,
}
impl TinkoffSpecificHeadersInterceptor {
    fn new(token: impl Display) -> Result<Self, Box<dyn Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "https://invest-public-api.tinkoff.ru:443";
    let channel = Endpoint::new(uri)?.connect().await?;
    let interceptor = TinkoffSpecificHeadersInterceptor::new(std::env::var("TOKEN")?)?;
    
    let mut client = UsersServiceClient::with_interceptor(channel, interceptor);
    let response = client.get_accounts(GetAccountsRequest {}).await?;
    let message = response.into_inner();
    let accounts = &message.accounts;
    for account in accounts {
        println!("{}\t{}", account.id, account.name);
    }
    Ok(())
}
