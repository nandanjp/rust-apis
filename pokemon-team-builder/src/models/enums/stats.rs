#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "stat")]
pub enum Stat {
    Attack,
    Defense,
    SpAttack,
    SpDefense,
    Speed,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum StatWithVal {
    Attack(i32),
    Defense(i32),
    SpAttack(i32),
    SpDefense(i32),
    Speed(i32),
}
