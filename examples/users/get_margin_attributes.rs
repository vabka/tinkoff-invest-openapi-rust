use tinkoff_invest_sdk::{TinkoffInvestClient, types::MoneyValue};

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut users_client = client.users();

    let accounts = users_client.get_accounts().await.unwrap();
    let first_account = &accounts[0];
    let account_id = first_account.id();
    let margin_attributes = users_client
        .get_margin_attributes(account_id)
        .await
        .unwrap();

    println!("Название счёта: {}", first_account.name());
    if let Some(MoneyValue { currency, amount }) = margin_attributes.liquid_portfolio() {
        println!("Ликвидный портфель: {amount} {currency}");
    }
    if let Some(MoneyValue { currency, amount }) = margin_attributes.minimal_margin() {
        println!("Минимальная маржа: {amount} {currency}");
    }
    if let Some(MoneyValue { currency, amount }) = margin_attributes.starting_margin() {
        println!("Начальная маржа: {amount} {currency}");
    }
    if let Some(funds_sufficiency_level) = margin_attributes.funds_sufficiency_level() {
        println!("Обеспечение: {}", funds_sufficiency_level);
    }
}
