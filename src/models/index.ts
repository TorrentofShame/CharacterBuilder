enum ABILITY {
  STR = "Strength",
  DEX = "Dexterity",
  CON = "Constitution",
  INT = "Intelligence",
  WIS = "Wisdom",
  CHA = "Charisma"
}

enum SKILL {
  ACROBATICS = "Acrobatics",
  ANIMAL_HANDLING = "Animal Handling",
  ARCANA = "Arcana",
  ATHLETICS = "Athletics",
  DECEPTION = "Deception",
  HISTORY = "History",
  INSIGHT = "Insight",
  INTIMIDATION = "Intimidation",
  INVESTIGATION = "Investigation",
  MEDICINE = "Medicine",
  NATURE = "Nature",
  PERCEPTION = "Perception",
  PERFORMANCE = "Performance",
  PERSUASION = "Persuasion",
  RELIGION = "Religion",
  SLEIGHT_OF_HAND = "Sleight of Hand",
  STEALTH = "Stealth",
  SURVIVAL = "Survival"
}

enum SIZE_CATEGORY {
  T = "Tiny",
  S = "Small",
  M = "Medium",
  L = "Large",
  H = "Huge",
  G = "Gargantuan"
}

interface Race {
  name: string;
  size: SIZE_CATEGORY;
  speed: {
    walk?: number,
    fly?: number,
    climb?: number,
    swim?: number
  }
  ability: Record<ABILITY, number>;
  armorClass: Array<number | ABILITY>; // Add items together
}

interface Character {
  name: string;
  race: Race;
}

type Patch =
  | {op: "add"; path: string; value: any}
  | {op: "remove"; path: string}
  | {op: "replace"; path: string; value: any}
  | {op: "copy"; path: string; from: string}
  | {op: "move"; path: string; from: string}
  | {op: "test"; path: string; value: any}

interface CharacterSheet {
  name: string;
  abilities: Record<ABILITY, number>;
  skills: Record<SKILL, number>;
}
