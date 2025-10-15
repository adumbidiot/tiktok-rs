use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

pub(crate) fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    struct Visitor<T>(PhantomData<T>);
    impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            value.parse().map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_str(Visitor::<T>(PhantomData))
}
