#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Unknown,
}
