use tinkoff_invest_sdk::TinkoffInvestClient;
use tinkoff_invest_sdk::{instruments::InstrumentRequest};
#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut instruments_client = client.instruments();
    let request = InstrumentRequest::Figi("BBG004730N88".to_string());
    let bond = instruments_client.bond_by(request).await.unwrap();
    println!("{:?}", bond);
}
