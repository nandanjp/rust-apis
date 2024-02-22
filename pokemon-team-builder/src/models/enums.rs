use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
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
    ScarlettViolet,
}
