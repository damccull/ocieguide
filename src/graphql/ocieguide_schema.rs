use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use regex::Regex;
use serde::{Deserialize, Serialize};
use slab::Slab;
use uuid::Uuid;

pub type OcieGuideSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct OcieGuide {
    pub ocie_item_data: HashMap<Uuid, usize>,
}
impl OcieGuide {
    pub fn new() -> Self {
        let mut items = Slab::new();

        let pt_belt = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("8465-01-444-1493".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("DA650B".to_string()).unwrap()),
            nomenclature: Some("BELT,HIGH VISIBILITY".to_string()),
            size: Some("YELLOW".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("5.57".parse::<f32>().unwrap_or(0.0f32)),
        });
        let a = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("7042-01-C10-5056".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("04071N".to_string()).unwrap()),
            nomenclature: Some("DIGITAL MUSIC DISPLAY SYSTEM WITH A".to_string()),
            size: Some("30GB".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("200.00".parse::<f32>().unwrap_or(0.0f32)),
        });
        let b = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("5835-01-557-9062".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("04071N".to_string()).unwrap()),
            nomenclature: Some("RECORDER-REPRODUCER,SOUND".to_string()),
            size: Some("8GB IPODS".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("213.00".parse::<f32>().unwrap_or(0.0f32)),
        });
        let c = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("8415-01-F00-0922".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("05008N".to_string()).unwrap()),
            nomenclature: Some("UNDERSHIRT, EXTREME COLD WEATHER".to_string()),
            size: Some("XXL-R BLACK".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("17.05".parse::<f32>().unwrap_or(0.0f32)),
        });
        let d = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("8415-01-501-7075".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("05008N".to_string()).unwrap()),
            nomenclature: Some("UNDERSHIRT,EXTREMECOLD WEATHER".to_string()),
            size: Some("M-R BLACK".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("17.32".parse::<f32>().unwrap_or(0.0f32)),
        });
        let e = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("8415-01-501-7077".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("05008N".to_string()).unwrap()),
            nomenclature: Some("UNDERSHIRT,EXTREMECOLD WEATHER".to_string()),
            size: Some("L-R BLACK".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("17.32".parse::<f32>().unwrap_or(0.0f32)),
        });
        let f = items.insert(OcieItem {
            id: Uuid::new_v4(),
            nsn: Some(NationalStockNumber::parse("8415-01-501-7113".to_string()).unwrap()),
            lin: Some(LineItemNumber::parse("05008N".to_string()).unwrap()),
            nomenclature: Some("UNDERSHIRT,EXTREMECOLD WEATHER".to_string()),
            size: Some("XL-R BLACK".to_string()),
            unit_of_issue: Some("EACH".to_string()),
            price: Some("17.51".parse::<f32>().unwrap_or(0.0f32)),
        });

        let ocie_item_data = HashMap::new();
        for (u, item) in items.into_iter() {
            ocie_item_data.insert(item.id.clone(), u);
        }

        Self { ocie_item_data }
    }

    pub fn item(&self, id: &str) -> Option<usize> {
        let id = Uuid::parse_str(id).unwrap_or_default();
        self.ocie_item_data.get(&id).cloned()
    }

    pub fn items(&self) -> Vec<usize> {
        self.ocie_item_data.values().cloned().collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OcieItem {
    pub id: Uuid,
    pub nsn: Option<NationalStockNumber>,
    pub lin: Option<LineItemNumber>,
    pub nomenclature: Option<String>,
    pub size: Option<String>,
    pub unit_of_issue: Option<String>,
    pub price: Option<f32>,
}
impl OcieItem {
    /// An `OcieItem` is valid only when it has at least one of:
    /// NSN, LIN, Nomenclature
    pub fn is_valid(&self) -> bool {
        self.nsn.is_some() || self.lin.is_some() || self.nomenclature.is_some()
    }
}
impl Default for OcieItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nsn: None,
            lin: None,
            nomenclature: None,
            size: None,
            unit_of_issue: None,
            price: None,
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
