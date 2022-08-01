mod lib;
use lib::tinkoff::invest::v1::GetAccountsRequest;
use lib::TinkoffInvestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN")?;
    let tinkoff_client = TinkoffInvestClient::connect(&token).await?;
    let mut client = tinkoff_client.users();
    let response = client.get_accounts(GetAccountsRequest {}).await?;
    let accounts = response.into_inner().accounts;
    for account in accounts {
        println!("{}\t{}", account.id, account.name);
    }
    Ok(())
}
