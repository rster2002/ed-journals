#[macro_export]
macro_rules! from_str_deserialize_impl {
    ($ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                FromStr::from_str(&s).map_err(|value| {
                    serde::de::Error::custom(format!("Failed to deserialize: got '{}'", value))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! try_from_deserialize_impl {
    ($f:ident => $ty:ident) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                let value = $f::deserialize(deserializer)?;

                $ty::try_from(value)
                    .map_err(|e| serde::de::Error::custom(format!("Failed to deserialize: got '{}'", value)))
            }
        }
    };
}
