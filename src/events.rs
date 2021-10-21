//! This module houses the events that are stored in event
//! streams to make up the reality of this application

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct OcieItemAggregate {
    version: usize,
    nsn: NationalStockNumber,
    lin: Option<LineItemNumber>,
    nomenclature: Option<String>,
    //name: String,
    size: Option<String>,
    menu: Option<String>,
}
impl OcieItemAggregate {
    pub fn new(
        lin: Option<LineItemNumber>,
        nsn: NationalStockNumber,
        nomenclature: Option<String>,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemAggregate {
        OcieItemAggregate {
            version: 1,
            lin,
            nsn,
            nomenclature,
            size,
            menu,
        }
    }

    pub fn update_information(
        lin: Option<LineItemNumber>,
        nsn: NationalStockNumber,
        nomenclature: Option<String>,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Updated(OcieItemData {
            lin,
            nsn,
            nomenclature,
            size,
            menu,
        })
    }

    pub fn remove(
        lin: Option<LineItemNumber>,
        nsn: NationalStockNumber,
        nomenclature: Option<String>,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Removed(OcieItemData {
            lin,
            nsn,
            nomenclature,
            size,
            menu,
        })
    }

    pub fn nsn(&self) -> &NationalStockNumber {
        &self.nsn
    }
    pub fn lin(&self) -> &Option<LineItemNumber> {
        &self.lin
    }
    pub fn nomenclature(&self) -> &Option<String> {
        &self.nomenclature
    }
    pub fn size(&self) -> &NationalStockNumber {
        &self.nsn
    }
    pub fn menu(&self) -> &NationalStockNumber {
        &self.nsn
    }
}

pub enum OcieItemEvent {
    Added(OcieItemData),
    Updated(OcieItemData),
    Removed(OcieItemData),
}
impl OcieItemEvent {
    // Not needed because information_updated will create if necessary
    // pub fn added(
    //     lin: &str,
    //     nsn: &str,
    //     nomenclature: &str,
    //     size: Option<String>,
    //     menu: Option<String>,
    // ) -> OcieItemEvent {
    //     OcieItemEvent::Added(OcieItemData {
    //         lin: lin.to_owned(),
    //         nsn: nsn.to_owned(),
    //         nomenclature: nomenclature.to_owned(),
    //         size,
    //         menu,
    //     })
    // }

    pub fn information_updated(
        lin: Option<LineItemNumber>,
        nsn: NationalStockNumber,
        nomenclature: Option<String>,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Updated(OcieItemData {
            lin,
            nsn,
            nomenclature,
            size,
            menu,
        })
    }

    pub fn removed(
        lin: Option<LineItemNumber>,
        nsn: NationalStockNumber,
        nomenclature: Option<String>,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Removed(OcieItemData {
            lin,
            nsn,
            nomenclature,
            size,
            menu,
        })
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcieItemData {
    pub nsn: NationalStockNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lin: Option<LineItemNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nomenclature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu: Option<String>,
}

trait Aggregate {
    type Item;

    fn version(&self) -> u64;
    fn apply(&self, evt: &Self::Item) -> Self
    where
        Self: Sized;
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
