//! Character
use super::{Asset, MetaData};
use crate::api::fetch_asset_definition;
use serde_derive::{Deserialize, Serialize};

/// The Character Spec that stores all necessary data to build a Character Sheet
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Character {
    /// The Character's metadata
    pub metadata: MetaData,
    /// The Character's spec
    pub spec: CharacterSpec,
}

/// To return all the [`CharacterAssetGrant`] of a struct
pub trait GetAllGrants {
    /// Returns all [`CharacterAssetGrant`] for self.
    fn all_grants(&self) -> Vec<CharacterAssetGrant>;
}

impl GetAllGrants for Character {
    fn all_grants(&self) -> Vec<CharacterAssetGrant> {
        // I doubt this is the most efficient way of doing this
        let mut grants: Vec<CharacterAssetGrant> = match &self.spec.assets.class {
            CharacterClassSpec::Single(class) => class
                .grants
                .clone()
                .into_iter()
                .flat_map(|g| g.all_grants())
                .collect(),
            CharacterClassSpec::Multi(classes) => classes
                .iter()
                .flat_map(|c| c.grants.clone())
                .flat_map(|g| g.all_grants())
                .collect(),
        };

        let mut race_grants: Vec<CharacterAssetGrant> = self
            .spec
            .assets
            .race
            .grants
            .clone()
            .into_iter()
            .flat_map(|g| g.all_grants())
            .collect();

        grants.append(&mut race_grants);

        grants
    }
}

impl GetAllGrants for CharacterAssetGrant {
    fn all_grants(&self) -> Vec<CharacterAssetGrant> {
        match self {
            CharacterAssetGrant::Trait { grants, .. } => grants
                .iter()
                .flat_map(|g| g.all_grants())
                .chain(std::iter::once(self.clone()))
                .collect(),
            CharacterAssetGrant::SubRace { grants, .. } => grants
                .iter()
                .flat_map(|g| g.all_grants())
                .chain(std::iter::once(self.clone()))
                .collect(),
            CharacterAssetGrant::SubClass { grants, .. } => grants
                .iter()
                .flat_map(|g| g.all_grants())
                .chain(std::iter::once(self.clone()))
                .collect(),
            CharacterAssetGrant::ASI(ASI::Feat { grants, .. }) => grants
                .iter()
                .flat_map(|g| g.all_grants())
                .chain(std::iter::once(self.clone()))
                .collect(),
            _ => vec![self.clone()],
        }
    }
}

/// Character Specification
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterSpec {
    /// The Ability Scores for a Character
    pub abilities: Abilities,
    /// The Character's assets
    #[serde(flatten)]
    pub assets: CharacterAssets,
}

impl Character {
    /// Returns a Character's level
    pub fn level(&self) -> i8 {
        match &self.spec.assets.class {
            CharacterClassSpec::Single(class) => class.level,
            CharacterClassSpec::Multi(classes) => classes.iter().fold(0, |a, e| a + e.level),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::assets::character;

    #[test]
    fn test_character_level_trait_single_class() {
        let ch = Character {
            metadata: MetaData {
                id: "uuid-lmao-lol".to_string(),
                name: "foobar".to_string(),
                notes: Some("yall are sick".to_string()),
                description: None,
                extra: Default::default(),
            },
            spec: CharacterSpec {
                abilities: Abilities {
                    strength: 9,
                    dexterity: 20,
                    constitution: 11,
                    intelligence: 11,
                    wisdom: 10,
                    charisma: 13,
                },
                assets: character::CharacterAssets {
                    class: CharacterClassSpec::Single(CharacterClass {
                        id: String::from("fighter"),
                        level: 1,
                        multiclass: false,
                        grants: vec![
                            CharacterAssetGrant::ASI(ASI::Ability(
                                String::from("strength"),
                                Some(String::from("dexterity")),
                            )),
                            CharacterAssetGrant::ASI(ASI::Feat {
                                id: String::from("this feat"),
                                grants: vec![],
                            }),
                        ],
                    }),
                    race: CharacterRace {
                        id: String::from("elf"),
                        grants: vec![],
                    },
                },
            },
        };

        assert_eq!(1, ch.level());
    }

    #[test]
    fn test_character_level_trait_multi_class() {
        let ch = Character {
            metadata: MetaData {
                id: "uuid-lmao-lol".to_string(),
                name: "foobar".to_string(),
                notes: Some("yall are sick".to_string()),
                description: None,
                extra: Default::default(),
            },
            spec: CharacterSpec {
                abilities: Abilities {
                    strength: 9,
                    dexterity: 20,
                    constitution: 11,
                    intelligence: 11,
                    wisdom: 10,
                    charisma: 13,
                },
                assets: character::CharacterAssets {
                    class: CharacterClassSpec::Multi(vec![
                        CharacterClass {
                            id: String::from("fighter"),
                            level: 2,
                            multiclass: false,
                            grants: vec![
                                CharacterAssetGrant::ASI(ASI::Ability(
                                    String::from("strength"),
                                    Some(String::from("dexterity")),
                                )),
                                CharacterAssetGrant::ASI(ASI::Feat {
                                    id: String::from("this feat"),
                                    grants: vec![],
                                }),
                            ],
                        },
                        CharacterClass {
                            id: String::from("paladin"),
                            level: 1,
                            multiclass: false,
                            grants: vec![
                                CharacterAssetGrant::ASI(ASI::Ability(
                                    String::from("strength"),
                                    Some(String::from("dexterity")),
                                )),
                                CharacterAssetGrant::ASI(ASI::Feat {
                                    id: String::from("this feat"),
                                    grants: vec![],
                                }),
                            ],
                        },
                    ]),
                    race: CharacterRace {
                        id: String::from("elf"),
                        grants: vec![],
                    },
                },
            },
        };

        assert_eq!(3, ch.level());
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// CharacterAssets
pub struct CharacterAssets {
    /// CharacterClassSpec
    pub class: CharacterClassSpec,
    /// CharacterRace
    pub race: CharacterRace,
}

/// CharacterClassSpec
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CharacterClassSpec {
    /// For Multi-Classed Characters
    Multi(Vec<CharacterClass>),
    /// For Single-Class Noobs
    Single(CharacterClass),
}

/// A Character's Class
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterClass {
    /// Unique ID
    pub id: String,
    /// The Class Level
    pub level: i8,
    /// If this class was chosen as a multi-class
    #[serde(default)]
    pub multiclass: bool,
    /// Assets that are granted to the Character by this class
    pub grants: Vec<CharacterAssetGrant>,
}

/// A Character's Race
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterRace {
    /// Unique ID
    pub id: String,
    /// Assets that are granted to the Character by this race
    pub grants: Vec<CharacterAssetGrant>,
}

/// Assets
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum CharacterAssetGrant {
    // This seems like a bad way to go about this lmao
    /// Grants a Proficiency to the Character
    Proficiency {
        /// Unique ID
        id: String,
    },
    /// Grants a Language to the Character
    Language {
        /// Unique ID
        id: String,
    },
    /// Grants a Feature to the Character
    Feature {
        /// Unique ID
        id: String,
    },
    /// Grants a Spell to the Character
    Spell {
        /// Unique ID
        id: String,
    },
    /// Grants a Size to the Character
    Size {
        /// Unique ID
        id: String,
    },
    /// Grants a Trait to the Character
    Trait {
        /// Unique ID
        id: String,
        /// Assets that are granted to the Character by this Trait
        grants: Vec<CharacterAssetGrant>,
    },
    /// Grants a SubRace to the Character
    #[serde(rename = "sub-race")]
    SubRace {
        /// Unique ID
        id: String,
        /// Assets that are granted to the Character by this SubRace
        grants: Vec<CharacterAssetGrant>,
    },
    /// Grants a SubClass to the Character
    #[serde(rename = "sub-class")]
    SubClass {
        /// Unique ID
        id: String,
        /// Assets that are granted to the Character by this SubClass
        grants: Vec<CharacterAssetGrant>,
    },
    /// Grants an ASI to the Character
    ASI(ASI),
    /// Grants Advantage to a roll to the Character
    Advantage {
        /// Unique ID of the roll to grant advantage to
        id: String,
    },
    /// Grants Disadvantage to a roll to the Character
    Disadvantage {
        /// Unique ID of the roll to grant disadvantage to
        id: String,
    },
    /// Adds a number to an AbilityScore
    AbilityScore {
        /// ID of the AbilityScore to modify
        id: String,
        /// Number to add to the AbilityScore
        add: i8,
    },
}

/// Trait to fetch the Asset definition from the api
pub trait FetchDefFromAPI<T> {
    /// Error
    type Err;
    /// Fetches this Asset's definition from the api
    fn fetch_def(&self) -> Result<T, Self::Err>;
}

impl FetchDefFromAPI<Asset> for CharacterAssetGrant {
    type Err = ();
    fn fetch_def(&self) -> Result<Asset, Self::Err> {
        match self {
            Self::Proficiency { id } => fetch_asset_definition("proficiency", id).map_err(|_| ()),
            _ => Err(()),
        }
    }
}

/// Ability Score Improvement
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ASI {
    /// Ability Scores to Improve
    Ability(String, Option<String>),
    /// Feat to get if not taking Ability Score Improvement
    Feat {
        /// Feat ID
        id: String,
        /// Assets granted to the Character by this Feat
        grants: Vec<CharacterAssetGrant>,
    },
}

/// A Character's Ability Scores
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Abilities {
    /// Strength
    pub strength: i8,
    /// Dexterity
    pub dexterity: i8,
    /// Constitution
    pub constitution: i8,
    /// Intelligence
    pub intelligence: i8,
    /// Wisdom
    pub wisdom: i8,
    /// Charisma
    pub charisma: i8,
}
