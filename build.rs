use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
    fs,
    io::{self, Write},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;

const PATH: &'static str = "./src/generated";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_path = Path::new(PATH);

    ensure_path(api_path)?;
    // TODO найти способ, как не запускать генерацию во время сборки в CI
    generate_code_from_contracts(api_path)?;
    generate_mod_rs(api_path)?;
    emit_error(api_path)?;
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
        r##"
#[path = "tinkoff.public.invest.api.contract.v1.rs"]
pub mod tinkoff_invest_v1;
pub mod errors;
"##,
    )
}

fn generate_code_from_contracts(api_path: &Path) -> io::Result<()> {
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
        )
}

#[derive(Deserialize)]
struct ErrorDescription {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    description: String,
}

fn emit_error(api_path: &Path) -> io::Result<()> {
    let file = fs::File::open("./investAPI/src/docs/errors/api_errors.json")?;
    let reader = std::io::BufReader::new(file);
    let errors: HashMap<String, ErrorDescription> = serde_json::from_reader(reader)?;

    let destination_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(api_path.join("errors.rs"))?;

    let mut writer = io::BufWriter::new(destination_file);
    emit_tinkoff_invest_error(&mut writer)?;
    emit_error_type(&mut writer, &errors)?;
    emit_messages(&mut writer, &errors)?;
    Ok(())
}

fn emit_messages(
    writer: &mut io::BufWriter<fs::File>,
    errors: &HashMap<String, ErrorDescription>,
) -> io::Result<()> {
    writer.write(
        r#"
fn get_description_by_code(code: &str) -> (String, String) {
    match code {"#
            .as_bytes(),
    )?;
    let br_regex = Regex::new(r"\s*</br>\s*").unwrap();
    let link_regex = Regex::new(r"\[[^\]]*\]\(([^\)]*)\)").unwrap();
    for (key, value) in errors {
        let message = &value.message;
        let description_without_br = br_regex.replace_all(&value.description, "\n");
        let description_with_unwrapped_links =
            link_regex.replace_all(&description_without_br, "$1");

        writer.write(
            format!(
                r###"
        "{key}" => (r"{message}".to_string(),
                    r##"{description_with_unwrapped_links}"##
                    .to_string()),"###
            )
            .as_bytes(),
        )?;
    }
    writer.write(
        r#"
        _ => ("N/A".to_string(), "N/A".to_string()),
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}

fn emit_tinkoff_invest_error(writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    writer.write(
        r#"
#[derive(Debug)]
pub struct TinkoffInvestError {
    formatted_message: String,
    error_type: ErrorType,
    description: String,
    code: String
}
"#
        .as_bytes(),
    )?;
    write!(
        writer,
        r#"
impl std::error::Error for TinkoffInvestError {{}}

impl std::fmt::Display for TinkoffInvestError {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "[{{}}] {{}}: {{}}", self.code, self.error_type, self.formatted_message)
    }}
}}

impl TinkoffInvestError {{
    pub fn formatted_message(&self) -> &str {{
        self.formatted_message.as_ref()
    }}
    
    pub fn error_type(&self) -> &ErrorType {{
        &self.error_type
    }}
    
    pub fn description(&self) -> &str {{
        self.description.as_ref()
    }}
    
    pub fn code(&self) -> &str {{
        self.code.as_ref()
    }}
}}
"#
    )?;
    Ok(())
}

fn emit_error_type(
    writer: &mut io::BufWriter<fs::File>,
    data: &HashMap<String, ErrorDescription>,
) -> Result<(), io::Error> {
    writer.write(
        r#"
impl ErrorType {
    pub fn from_code(code: &str) -> Self {
        match code {"#
            .as_bytes(),
    )?;
    let mut error_types = HashSet::new();
    let mut error_type_map = HashMap::new();
    for (key, value) in data {
        let type_name = upper_snake_case_to_pascal_case(&value.error_type);

        write!(
            writer,
            r#"
            "{key}" => Self::{type_name},"#
        )?;
        if !error_type_map.contains_key(&type_name) {
            error_type_map.insert(type_name.clone(), value.error_type.clone());
        }
        error_types.insert(type_name);
    }
    writer.write(
        r#"
            _ => Self::Unknown,
        }
    }
}
"#
        .as_bytes(),
    )?;
    writer.write(
        r#"
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum ErrorType {"#
            .as_bytes(),
    )?;
    for error_type_name in &error_types {
        writer.write(
            format!(
                r#"
    {error_type_name},"#
            )
            .as_bytes(),
        )?;
    }
    writer.write(
        r#"
    Unknown,
}
"#
        .as_bytes(),
    )?;

    emit_display(writer, &error_types, &error_type_map)?;
    Ok(())
}

// impl std::error::Error for TinkoffInvestError {
// }

// impl std::fmt::Display for TinkoffInvestError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[{}] {}: {}", self.code, self.error_type, self.formatted_message)
//     }
// }

fn emit_display(
    writer: &mut io::BufWriter<fs::File>,
    error_types: &HashSet<String>,
    error_type_map: &HashMap<String, String>,
) -> io::Result<()> {
    write!(
        writer,
        r#"
impl std::fmt::Display for ErrorType {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match self {{"#
    )?;
    for error in error_types {
        let full_name = error_type_map
            .get(error)
            .map(String::as_str)
            .unwrap_or("UNKNOWN");
        write!(
            writer,
            r#"
            Self::{error} => write!(f, "{full_name}"),"#
        )?;
    }
    write!(
        writer,
        r#"
            _ => write!(f, "UNKNOWN"),
        }}
    }}
}}
"#
    )?;
    Ok(())
}

fn upper_snake_case_to_pascal_case(upper_sake_cased: &str) -> String {
    upper_sake_cased
        .split('_')
        .map(|word| {
            let (head, tail) = word.split_at(1);
            format!("{}{}", head, tail.to_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod upper_snake_case_to_pascal_case_tests {
    use crate::upper_snake_case_to_pascal_case;

    #[test]
    fn test_single_word() {
        let word = "WORD";
        let result = upper_snake_case_to_pascal_case(word);
        assert_eq!("Word", result);
    }
    #[test]
    fn test_multiple_words() {
        let input = "RANDOM_VERY_LONG_IDENTIFIER";
        let result = upper_snake_case_to_pascal_case(input);
        assert_eq!("RandomVeryLongIdentifier", result);
    }
}
