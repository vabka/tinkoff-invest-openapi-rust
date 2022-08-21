use tinkoff_invest_sdk::chrono::*;
use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();

    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut instruments_client = client.instruments();

    let today = Utc::today();
    let week_forward = today + Duration::days(6);
    let schedules = instruments_client
        .trading_schedules(String::from("moex"), today..=week_forward)
        .await
        .unwrap();
    for schedule in schedules {
        println!("{}", schedule.exchange());
        for day in schedule.days().iter().filter(|day| day.is_trading_day()) {
            println!(
                "{}\t{:?}\t{:?}\t{:?}",
                day.date(),
                day.opening_auction_start_time(),
                day.trading_time(),
                day.closing_auction_end_time()
            )
        }
        println!("");
    }
}
