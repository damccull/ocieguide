use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcieItem {
    pub id: Uuid,
    pub nsn: Option<NationalStockNumber>,
    pub lin: Option<LineItemNumber>,
    pub nomenclature: Option<String>,
    pub size: Option<String>,
    pub menu: Option<String>,
}
impl Default for OcieItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nsn: None,
            lin: None,
            nomenclature: None,
            size: None,
            menu: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NationalStockNumber(String);
impl NationalStockNumber {
    pub fn parse(value: String) -> Result<Self, String> {
        let nsn_regex = Regex::new(r#"^\d{4}-\d{2}-\d{3}-\d{4}$"#).unwrap();
        if !nsn_regex.is_match(&value) {
            return Err("The NSN was not properly formatted.".to_string());
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for NationalStockNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Default for NationalStockNumber {
    fn default() -> Self {
        NationalStockNumber("0000-00-000-0000".to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemNumber(String);
impl LineItemNumber {
    pub fn parse(value: String) -> Result<Self, String> {
        let lin_regex = Regex::new(r#"^[a-zA-Z0-9]{1}\d{5}$"#).unwrap();
        if !lin_regex.is_match(&value) {
            return Err("The LIN was not properly formatted.".to_string());
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for LineItemNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
