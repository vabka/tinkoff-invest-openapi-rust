use tinkoff_invest_sdk::instruments::InstrumentRequest;
use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut instruments_client = client.instruments();
    let request = InstrumentRequest::Figi("BBG00R05JT04".to_string());
    let bond = instruments_client.bond_by(request).await.unwrap().unwrap();
    println!("{}", bond.placement_date().unwrap());
    // println!("{:#?}", bond);
}
