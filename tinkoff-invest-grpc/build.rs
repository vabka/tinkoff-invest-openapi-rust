use std::{path::Path, io, fs};

const PATH: &'static str = "./src/generated";
const PROTO_PATH: &'static str = "../investAPI";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = Path::new(PROTO_PATH);
    let api_path = Path::new(PATH);

    ensure_path(&api_path)?;
    // TODO найти способ, как не запускать генерацию во время сборки в CI
    generate_code_from_contracts(&api_path, &proto_path)?;
    generate_mod_rs(&api_path)?;
    Ok(())
}

fn ensure_path(api_path: &Path) -> io::Result<()> {
    if api_path.exists() {
        fs::remove_dir_all(api_path)?;
    }
    std::fs::create_dir(api_path)
}

fn generate_mod_rs(api_path: &Path) -> io::Result<()> {
    std::fs::write(
        api_path.join("mod.rs"),
        r##"#[path = "tinkoff.public.invest.api.contract.v1.rs"]
pub mod tinkoff_invest_v1;
"##,
    )
}

fn generate_code_from_contracts(api_path: &Path, proto_path: &Path) -> io::Result<()> {
    let contracts_path = proto_path.join("src/docs/contracts/");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(api_path)
        .compile(
            &[
                contracts_path.join("common.proto"),
                contracts_path.join("instruments.proto"),
                contracts_path.join("marketdata.proto"),
                contracts_path.join("operations.proto"),
                contracts_path.join("orders.proto"),
                contracts_path.join("sandbox.proto"),
                contracts_path.join("stoporders.proto"),
                contracts_path.join("users.proto"),
            ],
            &[contracts_path],
        )
}
