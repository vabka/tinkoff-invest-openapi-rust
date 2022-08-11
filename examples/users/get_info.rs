use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut users_client = client.users();

    let info = users_client.get_info().await.unwrap();

    println!(
        r"
Тариф?: {} 
Премиум?: {}
Квалифицированный инвестор?: {}
Квалифицирован для работы с:
{}
",
        info.tariff(),
        info.is_premium(),
        info.is_qualified(),
        info.qualified_for_work_with()
            .into_iter()
            .map(|i| format!("  {}", i))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
