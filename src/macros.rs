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

// #[macro_export]
// macro_rules! map_aliases {
//     ($t:ty => {$($l:literal => $i:tt)+,}) => {
//         impl FromStr for $t {
//             type Err = ();
//
//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 match s {
//                     $($l => Ok($t::$i))+,
//                     _ => Err(()),
//                 }
//             }
//         }
//     };
// }
//
// struct A;
//
// impl FromStr for A {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }
