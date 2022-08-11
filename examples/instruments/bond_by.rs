use tinkoff_invest_sdk::instruments::InstrumentRequest;
use tinkoff_invest_sdk::TinkoffInvestClient;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    
    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut instruments_client = client.instruments();    
    
    let bond_figi = "BBG00R05JT04".to_string(); // Черкизово выпуск 2
    let request = InstrumentRequest::Figi(bond_figi);
    
    // Игнорируем ошибку через unwrap. Не рекомендуется делать так в production
    let bond = instruments_client.bond_by(request).await.unwrap().unwrap();
    
    let name = bond.name();
    let price = bond.placement_price().unwrap();
    println!("{}\t{} {}", &name, &price.amount, &price.currency);
}
