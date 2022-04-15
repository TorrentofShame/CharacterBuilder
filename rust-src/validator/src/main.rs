//! Validation for Character Builder
#![warn(missing_docs)]
//use std::env;
use std::{io, fs};
use std::{path::Path, process};
use validator::assets::*;
use validator::assets::character::{
    Abilities, Character, CharacterAssetGrant, CharacterClass, CharacterClassSpec, CharacterRace,
    ASI, CharacterSpec,
};
//use validator::character_sheet::*;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let filename = &args[1];

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
                    grants: vec![CharacterAssetGrant::ASI(
                        ASI::Ability(
                            String::from("strength"),
                            None,
                        )
                    )],
                }),
                race: CharacterRace {
                    id: String::from("elf"),
                    grants: vec![],
                },
            },
        },
    });

    //if let Asset::Character(c) = &expected {
        //if let Spec::Character{assets, ..} = &c.spec {
            //match &assets.class {
                //CharacterClassSpec::Single(class) => {
                    //if let CharacterAssetGrant::ASI(asi) = &class.grants[0] {
                        //match asi {
                            //ASI::Ability(y, x) => {
                            //},
                            //_ => {},
                        //}
                    //}
                //},
                //CharacterClassSpec::Multi(_classes) => {
                //},
            //}
        //}
    //}

    let fp = fs::File::create(&Path::new(filename)).unwrap();

    let writer = io::BufWriter::new(fp);

    let v = serde_yaml::to_writer(writer, &expected);


    //let v = validator::read_asset(&Path::new(filename));

    if let Err(e) = v {
        println!("Oops, somebody fucked up! {}", e);

        process::exit(1);
    }

    println!("written!");
    //println!("des: {:?}", v.unwrap());
}
