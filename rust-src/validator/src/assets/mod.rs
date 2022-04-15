//! Assets
pub mod character;
pub mod class;

use std::collections::HashMap;
use my_macros::SelectEnum;
use serde_derive::{Deserialize, Serialize};

/// MetaData
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MetaData {
    /// Unique Id for this Asset
    pub id: String,
    /// Pretty Name to be displayed
    pub name: String,
    /// User-written notes
    pub notes: Option<String>,
    /// A Description
    pub description: Option<String>,
    /// Any other fields as needed
    #[serde(flatten, default)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// Asset Grants
#[derive(Debug, PartialEq, Serialize, Deserialize, SelectEnum)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Grant {
    /// Proficiency
    Proficiency {
        /// Unique ID
        id: String
    }
}

/// Spec
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spec {
    /// Class Spec
    Class {
        /// Setters for Class Spec
        set: Setter,
        /// Grants for Class Spec
        grant: Vec<Grant>,
        /// Selects for Class Spec
        select: Vec<Select>,
    },
}

/// Die
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Die { // FIXME: Setup better way to deserialize dice & dice equations
    /// D4
    D4,
    /// D6
    D6,
    /// D8
    D8,
    /// D10
    D10,
    /// D12
    D12,
    /// D20
    D20,
}

/// Setter
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Setter { // TODO: have set field in Spec be deseriealized as a HS of enums
    /// Class Setter
    Class {
        /// Class Setter Hit Dice
        #[serde(rename = "hit-dice")]
        hit_dice: Die,
    },
}

/// Asset
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Asset {
    /// Character Asset
    Character (self::character::Character),
    /// Class Asset
    Class {
        /// MetaData
        metadata: MetaData,
        /// Spec
        spec: Spec,
    },
    /// Proficiency Asset
    Proficiency {
        /// MetaData
        metadata: MetaData,
    },
    /// Language Asset
    Language {
        /// MetaData
        metadata: MetaData,
    },
}
