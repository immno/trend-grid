use serde::de::{Unexpected, Visitor};
use serde::Deserializer;
use std::fmt;

pub fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64Visitor;

    impl<'de> Visitor<'de> for F64Visitor {
        type Value = f64;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representation of a f64")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.is_empty() {
                Ok(0.0)
            } else {
                v.parse::<f64>().map_err(|_| {
                    E::invalid_value(Unexpected::Str(v), &"a string representation as f64")
                })
            }
        }
    }
    deserializer.deserialize_str(F64Visitor)
}
