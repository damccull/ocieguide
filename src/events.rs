//! This module houses the events that are stored in event
//! streams to make up the reality of this application

use regex::Regex;

#[derive(Debug)]
pub struct OcieItemAggregate {
    version: usize,
    pub lin: String,
    pub nsn: NationalStockNumber,
    pub nomenclature: String,
    //pub name: String,
    pub size: Option<String>,
    pub menu: Option<String>,
}
impl OcieItemAggregate {
    pub fn new(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemAggregate {
        OcieItemAggregate {
            version: 1,
            lin: LineItemNumber::parse(lin.to_string()).unwrap(),
            nsn: NationalStockNumber::parse(nsn.to_string()).unwrap(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        }
    }

    pub fn update(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Updated(OcieItemData {
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        })
    }

    pub fn remove(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Removed(OcieItemData {
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        })
    }
}

pub enum OcieItemEvent {
    Added(OcieItemData),
    Updated(OcieItemData),
    Removed(OcieItemData),
}
impl OcieItemEvent {
    pub fn added(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Added(OcieItemData {
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        })
    }

    pub fn updated(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Updated(OcieItemData {
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        })
    }

    pub fn removed(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemEvent {
        OcieItemEvent::Removed(OcieItemData {
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        })
    }
}

pub struct OcieItemData {
    pub lin: String,
    pub nsn: String,
    pub nomenclature: String,
    //pub name: String,
    pub size: Option<String>,
    pub menu: Option<String>,
}

trait Aggregate {
    type Item;

    fn version(&self) -> u64;
    fn apply(&self, evt: &Self::Item) -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
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

#[derive(Debug)]
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
