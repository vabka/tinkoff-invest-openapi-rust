use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    
    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut users_client = client.users();
    
    let accounts = users_client.get_accounts().await.unwrap();
    for account in accounts {
        println!("{}\t{:?}\t{:?}\t{}", account.name(), account.account_type(), account.access_level(), account.opened_date().unwrap())
    }
}
