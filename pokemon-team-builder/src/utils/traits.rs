pub trait SerDeserEnum {
    type Error;
    fn to_str(&self) -> &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait IntoSerial {
    type Serial;
    fn to_serial(&self) -> Self::Serial;
}
