#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "game")]
pub enum Game {
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
