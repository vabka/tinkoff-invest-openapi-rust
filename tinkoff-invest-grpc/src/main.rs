use tinkoff_invest_grpc::{
    api::{CloseSandboxAccountRequest, GetAccountsRequest, OpenSandboxAccountRequest},
    TinkoffInvestClient,
};
use tonic::Status;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN")?;
    let tinkoff_client = TinkoffInvestClient::connect(&token).await?;
    let mut client = tinkoff_client.sandbox();
    let response = client
        .open_sandbox_account(OpenSandboxAccountRequest {})
        .await;
    match response {
        Ok(response) => {
            println!("{response:#?}")
        }
        Err(e) => {
            println!("{e:#?}");
        }
    }

    let response = client
        .close_sandbox_account(CloseSandboxAccountRequest {
            account_id: "I guarantee that this account is not exists".to_string(),
        })
        .await;
    println!("{response:#?}");
    Ok(())
}
