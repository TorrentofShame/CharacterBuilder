//! Validation for Character Builder
#![warn(missing_docs)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
extern crate serde_yaml;

pub mod api;
pub mod character_sheet;
pub mod assets;
use crate::assets::*;

/// Validation Error
#[derive(Debug, Clone)]
pub struct ValidatorError;

/// Reads in an asset
pub fn read_asset(path: &::std::path::Path) -> Result<Asset, ValidatorError> {
    let f = ::std::fs::File::open(path).unwrap();

    let reader = ::std::io::BufReader::new(f);

    let deserialized: Asset = ::serde_yaml::from_reader(reader).unwrap();

    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use crate::assets::character::{
        Abilities, Character, CharacterAssetGrant, CharacterClass, CharacterClassSpec,
        CharacterRace, ASI, CharacterSpec,
    };

    use super::*;

    #[test]
    fn test_character_read() {
        let file_path = ::std::path::Path::new("mockChar.yml");

        let expected = Asset::Character(Character {
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
                            CharacterAssetGrant::ASI(
                                ASI::Ability(
                                    String::from("strength"),
                                    Some(String::from("dexterity")),
                                )
                            ),
                            CharacterAssetGrant::ASI(
                                ASI::Feat{
                                    id: String::from("this feat"),
                                    grants: vec![],
                                },
                            )
                        ],
                    }),
                    race: CharacterRace {
                        id: String::from("elf"),
                        grants: vec![],
                    },
                },
            },
        });

        assert_eq!(expected, read_asset(file_path).unwrap());
    }

    #[test]
    fn test_class_read() {
        let file_path = ::std::path::Path::new("mockClass.yml");

        let expected = Asset::Class {
            metadata: MetaData {
                id: "fighter".to_string(),
                name: "Fighter".to_string(),
                description: Some("Lorem ipsum dolor sit amet.\n".to_string()),
                notes: None,
                extra: Default::default(),
            },
            spec: Spec::Class {
                set: Setter::Class { hit_dice: Die::D10 },
                grant: vec![
                    Grant::Proficiency {
                        id: "armor-light".to_string(),
                    },
                    Grant::Proficiency {
                        id: "armor-medium".to_string(),
                    },
                ],
                select: vec![Select::Proficiency(SelectVariant {
                    name: "Skill Proficiency".to_string(),
                    number: 2,
                    id: vec![
                        String::from("skill-acrobatics"),
                        String::from("skill-animal-handling"),
                        String::from("skill-athletics"),
                        String::from("skill-history"),
                        String::from("skill-insight"),
                        String::from("skill-intimidation"),
                        String::from("skill-perception"),
                        String::from("skill-survival"),
                    ],
                })],
            },
        };

        assert_eq!(expected, read_asset(file_path).unwrap());
    }
}
