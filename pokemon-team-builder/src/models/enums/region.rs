#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "region")]
pub enum Region {
    Kanto,
    Johto,
    Hoenn,
    Hisui,
    Sinnoh,
    Unova,
    Kalos,
    Alola,
    Galar,
    Paldea,
}
