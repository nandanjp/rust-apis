use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

impl Gender {
    pub fn to_string(&self) -> String {
        match self {
            Gender::Male => String::from("male"),
            Gender::Female => String::from("female"),
            Gender::Unknown => String::from("unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Grass,
    Flying,
    Fighting,
    Poison,
    Electric,
    Ground,
    Rock,
    Psychic,
    Ice,
    Bug,
    Ghost,
    Steel,
    Dragon,
    Dark,
    Fairy,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Normal => String::from("normal"),
            Type::Fire => String::from("fire"),
            Type::Water => String::from("water"),
            Type::Grass => String::from("grass"),
            Type::Flying => String::from("flying"),
            Type::Fighting => String::from("fighting"),
            Type::Poison => String::from("poison"),
            Type::Electric => String::from("electric"),
            Type::Ground => String::from("ground"),
            Type::Rock => String::from("rock"),
            Type::Psychic => String::from("psychic"),
            Type::Ice => String::from("ice"),
            Type::Bug => String::from("bug"),
            Type::Ghost => String::from("ghost"),
            Type::Steel => String::from("steel"),
            Type::Dragon => String::from("dragon"),
            Type::Dark => String::from("dark"),
            Type::Fairy => String::from("fairy"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Stat {
    Attack,
    Defense,
    SpAttack,
    SpDefense,
    Speed,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OriginGame {
    RedBlue,
    GoldSilver,
    RubySapphire,
    DiamondPearl,
    BlackWhite,
    XY,
    SunMoon,
    SwordShield,
    ScarletViolet,
}

impl OriginGame {
    pub fn to_string(&self) -> String {
        match self {
            OriginGame::RedBlue => String::from("red-blue"),
            OriginGame::GoldSilver => String::from("gold-silver"),
            OriginGame::RubySapphire => String::from("ruby-sapphire"),
            OriginGame::DiamondPearl => String::from("diamond-pearl"),
            OriginGame::BlackWhite => String::from("black-white"),
            OriginGame::XY => String::from("x-y"),
            OriginGame::SunMoon => String::from("sun-moon"),
            OriginGame::SwordShield => String::from("sword-shield"),
            OriginGame::ScarletViolet => String::from("scarlett-violet"),
        }
    }
}
