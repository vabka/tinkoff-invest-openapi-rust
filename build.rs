use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_path = Path::new("./src/api");

    if !api_path.exists() {
        std::fs::create_dir(api_path)?;
    }

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(api_path)
        .compile(
            &[
                "./investAPI/src/docs/contracts/common.proto",
                "./investAPI/src/docs/contracts/instruments.proto",
                "./investAPI/src/docs/contracts/marketdata.proto",
                "./investAPI/src/docs/contracts/operations.proto",
                "./investAPI/src/docs/contracts/orders.proto",
                "./investAPI/src/docs/contracts/sandbox.proto",
                "./investAPI/src/docs/contracts/stoporders.proto",
                "./investAPI/src/docs/contracts/users.proto",
            ],
            &["./investAPI/src/docs/contracts/"],
        )?;
    std::fs::rename(
        api_path.join("tinkoff.public.invest.api.contract.v1.rs"),
        api_path.join("tinkoff_public_invest_api_contract_v1.rs"),
    )?;

    std::fs::remove_file(api_path.join("google.protobuf.rs"))?;
    Ok(())
}
