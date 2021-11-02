use bigdecimal::BigDecimal;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct OcieItem {
    pub(crate) id: Uuid,
    pub(crate) nsn: NationalStockNumber,
    pub(crate) lin: LineItemNumber,
    pub(crate) nomenclature: String,
    pub(crate) size: Option<String>,
    pub(crate) unit_of_issue: Option<String>,
    pub(crate) price: Option<BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NationalStockNumber(String);
impl NationalStockNumber {
    pub fn parse(value: String) -> Result<Self, anyhow::Error> {
        let nsn_regex = Regex::new(r#"^\d{4}-\d{2}-\d{3}-\d{4}$"#).unwrap();
        if !nsn_regex.is_match(&value) {
            return Err(anyhow::anyhow!(
                "The NSN was not properly formatted.".to_string()
            ));
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
    pub fn parse(value: String) -> Result<Self, anyhow::Error> {
        let lin_regex = Regex::new(r#"^[a-zA-Z0-9]{1}\d{5}$"#).unwrap();
        if !lin_regex.is_match(&value) {
            return Err(anyhow::anyhow!(
                "The LIN was not properly formatted.".to_string()
            ));
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for LineItemNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
