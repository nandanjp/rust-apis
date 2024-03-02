use serde::Serialize;

pub trait IntoSerializable<T>
where
    T: Serialize,
{
    fn to_serial(self) -> T;
}

pub trait SerializeEnum<T> {
    type Error;
    fn to_string(&self) -> &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
