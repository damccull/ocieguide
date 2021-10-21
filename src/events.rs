//! This module houses the events that are stored in event
//! streams to make up the reality of this application

#[derive(Debug)]
pub struct OcieItemAggregate {
    version: usize,
    pub lin: String,
    pub nsn: String,
    pub nomenclature: String,
    //pub name: String,
    pub size: Option<String>,
    pub menu: Option<String>,
}
impl OcieItemAggregate {
    fn new(
        lin: &str,
        nsn: &str,
        nomenclature: &str,
        size: Option<String>,
        menu: Option<String>,
    ) -> OcieItemAggregate {
        OcieItemAggregate {
            version: 1,
            lin: lin.to_owned(),
            nsn: nsn.to_owned(),
            nomenclature: nomenclature.to_owned(),
            size,
            menu,
        }
    }

    fn update(
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

    fn remove(
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
    fn added(
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

    fn updated(
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

    fn removed(
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
