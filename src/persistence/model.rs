use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

//TODO: Remove this dead code allowance
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OcieItem {
    pub id: Uuid,
    pub nsn: NationalStockNumber,
    pub lin: LineItemNumber,
    pub nomenclature: String,
    pub size: Option<String>,
    pub unit_of_issue: Option<String>,
    pub price: Option<BigDecimal>,
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
