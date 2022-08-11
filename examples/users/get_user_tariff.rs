use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut users_client = client.users();

    let tariff = users_client.get_user_tariff().await.unwrap();
    for unary_limit in tariff.unary_limits() {
        println!("{} req/min", unary_limit.limit_per_minute());
        println!("{}", unary_limit.methods().join("\n"));
        println!("----");
    }
    println!("================================================================");

    for stream_limit in tariff.stream_limits() {
        println!("{} connections", stream_limit.limit());
        println!("{}", stream_limit.streams().join("\n"));
        println!("----");
    }
}
