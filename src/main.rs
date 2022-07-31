mod contracts;
use contracts::tinkoff_public_invest_api_contract_v1::*;
use contracts::tinkoff_public_invest_api_contract_v1::users_service_client::*;
use tonic::{service::Interceptor, transport::Endpoint};

#[derive(Clone)]
struct TinkoffSpecificHeadersInterceptor {
    token: String,
    app_name: Option<String>,
}

impl Interceptor for TinkoffSpecificHeadersInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        {
            let metadata = request.metadata_mut();
            let authorization_value = format!("Bearer {}", self.token)
                .try_into()
                .expect("token value");
            metadata.insert("authorization", authorization_value);
            if let Some(app_name) = &self.app_name {
                let app_name = app_name.try_into().expect("app name");
                metadata.insert("x-app-name", app_name);
            }
        }
        Ok(request)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interceptor = TinkoffSpecificHeadersInterceptor {
        token: std::env::var("TOKEN")?,
        app_name: None,
    };

    let uri = "https://invest-public-api.tinkoff.ru:443";

    let channel = Endpoint::new(uri)?.connect().await?;

    let mut client =
        UsersServiceClient::with_interceptor(channel, interceptor);
    let response = client.get_accounts(GetAccountsRequest {}).await?;
    let message = response.into_inner();
    let accounts = &message.accounts;
    for account in accounts {
        println!("{}\t{}", account.id, account.name);
    }
    Ok(())
}
