use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, Write},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;

const PATH: &'static str = "./src/generated";
const PROTO_PATH: &'static str = "../investAPI";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = Path::new(PROTO_PATH);
    let api_path = Path::new(PATH);
    ensure_path(&api_path)?;
    generate_errors_rs(&api_path, &proto_path)?;
    generate_mod_rs(&api_path)?;
    Ok(())
}

fn generate_mod_rs(api_path: &Path) -> io::Result<()> {
    std::fs::write(
        api_path.join("mod.rs"),
        r##"pub mod errors;
"##,
    )
}

fn ensure_path(api_path: &Path) -> io::Result<()> {
    if api_path.exists() {
        fs::remove_dir_all(api_path)?;
    }
    std::fs::create_dir(api_path)
}


#[derive(Deserialize)]
struct ErrorDescription {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    description: String,
}

fn generate_errors_rs(api_path: &Path, proto_path: &Path) -> io::Result<()> {
    let errors_json_path = proto_path.join("src/docs/errors/api_errors.json");
    println!("cargo:rerun-if-changed={}", errors_json_path.display());


    let file = fs::File::open(errors_json_path)?;
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
    write!(
        writer,
        r#"
fn get_description_by_code(code: &str) -> (String, String) {{
    match code {{"#
    )?;
    let br_regex = Regex::new(r"\s*</br>\s*").unwrap();
    let link_regex = Regex::new(r"\[[^\]]*\]\(([^\)]*)\)").unwrap();
    for (key, value) in errors {
        let message = &value.message;
        let description_without_br = br_regex.replace_all(&value.description, "\n");
        let description_with_unwrapped_links =
            link_regex.replace_all(&description_without_br, "$1");

        write!(
            writer,
            r###"
        "{key}" => (r"{message}".to_string(),
                    r##"{description_with_unwrapped_links}"##
                    .to_string()),"###
        )?;
    }
    write!(
        writer,
        r#"
        _ => ("N/A".to_string(), "N/A".to_string()),
    }}
}}
"#
    )
}

fn emit_tinkoff_invest_error(writer: &mut io::BufWriter<fs::File>) -> Result<(), io::Error> {
    write!(
        writer,
        r#"#[derive(Debug)]
pub struct TinkoffInvestError {{
    formatted_message: String,
    error_type: ErrorType,
    description: String,
    code: String
}}

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

impl From<tonic::Status> for TinkoffInvestError {{
    fn from(status: tonic::Status) -> TinkoffInvestError {{
        let code = status.message();
        let err = ErrorType::from_code(code);
        let (message, description) = get_description_by_code(code);
        TinkoffInvestError {{
            code: code.to_string(),
            error_type: err,
            description: description,
            formatted_message: message
        }}
    }}
}}
"#
    )
}

fn emit_error_type(
    writer: &mut io::BufWriter<fs::File>,
    data: &HashMap<String, ErrorDescription>,
) -> Result<(), io::Error> {
    write!(
        writer,
        r#"
impl ErrorType {{
    pub fn from_code(code: &str) -> Self {{
        match code {{"#
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
    write!(
        writer,
        r#"
            _ => Self::Unknown,
        }}
    }}
}}
"#
    )?;
    write!(
        writer,
        r"
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum ErrorType {{"
    )?;
    for error_type_name in &error_types {
        write!(
            writer,
            r#"
    {error_type_name},"#
        )?;
    }
    write!(
        writer,
        r#"
    Unknown,
}}
"#
    )?;

    emit_display(writer, &error_types, &error_type_map)?;
    Ok(())
}

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