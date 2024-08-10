#[macro_export]
#[doc(hidden)]
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
#[doc(hidden)]
macro_rules! try_from_deserialize_impl {
    ($f:ident => $ty:ident) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = $f::deserialize(deserializer)?;

                $ty::try_from(value).map_err(|_e| {
                    serde::de::Error::custom(format!("Failed to deserialize: got '{}'", value))
                })
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! deserialize_in_order_impl {
    ($ty:ident => $($i:ident $l:tt $f:ident,)+) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(untagged)]
                enum Input {
                    $($i($f),)+
                };

                let input = Input::deserialize(deserializer)?;

                match input {
                    $(
                        Input::$i($f) => $crate::deserialize_in_order_entry!($ty => $i $l $f),
                    )+
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! deserialize_in_order_entry {
    ($ty:ident => $i:ident ? $f:ident) => {
        $ty::try_from($f).map_err(|_e| serde::de::Error::custom(format!("Failed to deserialize")))
    };

    ($ty:ident => $i:ident # $f:ident) => {
        $ty::from_str(&$f).map_err(|_e| serde::de::Error::custom(format!("Failed to deserialize")))
    };

    ($ty:ident => $i:ident ! $f:ident) => {
        return Ok($ty::from($f))
    };
}
