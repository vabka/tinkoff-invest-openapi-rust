mod api;
use api::tinkoff_public_invest_api_contract_v1::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tinkoff_client = api::TinkoffInvestClient::connect(std::env::var("TOKEN")?.as_str()).await?;
    let mut client = tinkoff_client.get_users_service_client();
    let response = client.get_accounts(GetAccountsRequest {}).await?;
    let message = response.into_inner();
    let accounts = &message.accounts;
    for account in accounts {
        println!("{}\t{}", account.id, account.name);
    }
    Ok(())
}
