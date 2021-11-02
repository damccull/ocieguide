use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct OcieItemEntity {
    id: Uuid,
    pub(crate) nsn: NationalStockNumber,
    pub(crate) lin: LineItemNumber,
    nomenclature: String,
    size: Option<String>,
    unit_of_issue: Option<String>,
    price: Option<f32>,
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
