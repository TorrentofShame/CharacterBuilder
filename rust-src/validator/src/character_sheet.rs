//! Module to deal with final Character Data, to be directly used where it is needed.
use crate::assets::{
    character::{Abilities, Character, CharacterAssetGrant, FetchDefFromAPI, GetAllGrants, ASI},
    Asset, MetaData,
};

/// This Struct is directly used when filling out fields in the ui character sheet
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSheet {
    level: i8,
    armor_class: i8,
    size: String,
    languages: Vec<String>,
    ability_scores: AbilityScores,
    proficiencies: Vec<MetaData>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct AbilityRoll {
    base: i8,
    mods: i8,
}

impl AbilityRoll {
    pub fn total(&self) -> i8 {
        self.base + self.mods
    }

    pub fn modifier(&self) -> i8 {
        (self.total() - 10) / 2
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct AbilityScores {
    strength: AbilityRoll,
    dexterity: AbilityRoll,
    constitution: AbilityRoll,
    intelligence: AbilityRoll,
    wisdom: AbilityRoll,
    charisma: AbilityRoll,
}

impl From<Abilities> for AbilityScores {
    fn from(value: Abilities) -> Self {
        Self {
            strength: AbilityRoll {
                base: value.strength,
                mods: 0,
            },
            dexterity: AbilityRoll {
                base: value.dexterity,
                mods: 0,
            },
            constitution: AbilityRoll {
                base: value.constitution,
                mods: 0,
            },
            intelligence: AbilityRoll {
                base: value.intelligence,
                mods: 0,
            },
            wisdom: AbilityRoll {
                base: value.wisdom,
                mods: 0,
            },
            charisma: AbilityRoll {
                base: value.charisma,
                mods: 0,
            },
        }
    }
}

impl AbilityScores {
    pub fn add_to_mod(&mut self, ability: &str, v: i8) {
        match ability {
            "strength" => self.strength.mods += v,
            "dexterity" => self.dexterity.mods += v,
            "constitution" => self.constitution.mods += v,
            "intelligence" => self.intelligence.mods += v,
            "wisdom" => self.wisdom.mods += v,
            "charisma" => self.charisma.mods += v,
            _ => {}
        };
    }
}

// TODO: Character Sheet Trait to write to rptok file, technically it's just a serde thing,
// might need the trait just to define the methods to get the macro text n shit, but ye.
// The rptok writer should probably be done in a separate lib.

/// This will not validate the Character struct, it will just convert it to a format
/// that the ui will be able to use.
impl TryFrom<Character> for CharacterSheet {
    type Error = ();

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        // First, we need to get all the base values not given by grant hell

        // Get Ability Scores
        let mut ability_scores: AbilityScores = value.spec.abilities.into();

        let mut size: String = String::default();

        let mut languages: Vec<String> = vec![];

        // Initially mut 0 to allow for things like armors to add their mods
        // before factoring in base ac
        let mut armor_class: i8 = 0;

        let mut proficiencies: Vec<MetaData> = vec![];

        // Get Character Level
        let level = value.level();

        // Now have grant hell work its magic!
        value.all_grants().iter().for_each(|grant| {
            match grant {
                CharacterAssetGrant::ASI(ASI::Ability(a, b)) => {
                    if let Some(bval) = b {
                        ability_scores.add_to_mod(a, 1);
                        ability_scores.add_to_mod(bval, 1);
                    } else {
                        ability_scores.add_to_mod(a, 2);
                    }
                }
                CharacterAssetGrant::Size { id } => {
                    size = String::from(id);
                }
                CharacterAssetGrant::AbilityScore { id, add: modifier } => {
                    ability_scores.add_to_mod(id, *modifier);
                }
                CharacterAssetGrant::Language { id } => {
                    languages.push(String::from(id));
                }
                CharacterAssetGrant::Proficiency { .. } => {
                    if let Asset::Proficiency { metadata } = grant.fetch_def().unwrap() {
                        proficiencies.push(metadata);
                    };
                }
                _ => {}
            };
        });

        // Calculate the base ac value and add it to w/ever we already have from grants
        armor_class += 10 + ability_scores.dexterity.modifier();

        Ok(Self {
            level,
            armor_class,
            languages,
            size,
            ability_scores,
            proficiencies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character;
    use crate::character::*;
    use crate::MetaData;

    /*
        #[test]
        fn test_ability_roll_total() {
            let roll = AbilityRoll {
                base: 1,
                mods: 3,
            };

            assert_eq!(roll.total(), 4);
        }

        #[test]
        fn test_ability_roll_total_negative() {
            let roll = AbilityRoll {
                base: 1,
                mods: -3,
            };

            assert_eq!(roll.total(), -2);
        }
    */

    #[test]
    fn test_ability_scores_add_to_mod() {
        let mut scores = AbilityScores {
            strength: AbilityRoll { base: 9, mods: 3 },
            dexterity: AbilityRoll { base: 20, mods: 1 },
            constitution: AbilityRoll { base: 11, mods: 4 },
            intelligence: AbilityRoll { base: 11, mods: 2 },
            wisdom: AbilityRoll { base: 10, mods: 0 },
            charisma: AbilityRoll { base: 13, mods: -1 },
        };

        scores.add_to_mod("strength", 1);
        scores.add_to_mod("dexterity", 0);
        scores.add_to_mod("constitution", -1);
        scores.add_to_mod("intelligence", 2);
        scores.add_to_mod("wisdom", 2);
        scores.add_to_mod("charisma", 1);

        assert_eq!(scores.strength.mods, 4);
        assert_eq!(scores.dexterity.mods, 1);
        assert_eq!(scores.constitution.mods, 3);
        assert_eq!(scores.intelligence.mods, 4);
        assert_eq!(scores.wisdom.mods, 2);
        assert_eq!(scores.charisma.mods, 0);
    }

    #[test]
    fn test_character_sheet_conversion() {
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
                                grants: vec![CharacterAssetGrant::AbilityScore {
                                    id: String::from("constitution"),
                                    add: 1,
                                }],
                            }),
                        ],
                    }),
                    race: CharacterRace {
                        id: String::from("elf"),
                        grants: vec![
                            CharacterAssetGrant::Size {
                                id: String::from("medium"),
                            },
                            CharacterAssetGrant::AbilityScore {
                                id: String::from("dexterity"),
                                add: 2,
                            },
                            CharacterAssetGrant::Language {
                                id: String::from("common"),
                            },
                            CharacterAssetGrant::Language {
                                id: String::from("elvish"),
                            },
                            CharacterAssetGrant::Trait {
                                id: String::from("keen-senses"),
                                grants: vec![CharacterAssetGrant::Proficiency {
                                    id: String::from("perception"),
                                }],
                            },
                        ],
                    },
                },
            },
        };

        let expected = CharacterSheet {
            level: 1,
            armor_class: 16,
            size: String::from("medium"),
            languages: vec![String::from("common"), String::from("elvish")],
            proficiencies: vec![MetaData {
                id: String::from("perception"),
                name: Default::default(),
                notes: None,
                description: None,
                extra: Default::default(),
            }],
            ability_scores: AbilityScores {
                strength: AbilityRoll { base: 9, mods: 1 },
                dexterity: AbilityRoll { base: 20, mods: 3 },
                constitution: AbilityRoll { base: 11, mods: 1 },
                intelligence: AbilityRoll { base: 11, mods: 0 },
                wisdom: AbilityRoll { base: 10, mods: 0 },
                charisma: AbilityRoll { base: 13, mods: 0 },
            },
        };

        assert_eq!(expected, CharacterSheet::try_from(ch).unwrap());
    }
}

/*
/// Implements (almost) everything needed to create a character sheet.
impl CharacterSheet {
    /// Returns the Character's level
    pub fn level(&self) -> i8 {
    }
    /// Returns a specific ability score as the tuple:
    /// (score, base, racial, mods)
    fn ability_score(&self, ability: String) -> (i8, i8, i8, i8) {
    }
}
*/
