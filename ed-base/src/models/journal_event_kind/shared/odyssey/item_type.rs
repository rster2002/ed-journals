use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ItemType {
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Data;")]
    Data,

    #[serde(
        alias = "Item",
        alias = "$MICRORESOURCE_CATEGORY_Item;",
    )]
    Goods,

    #[serde(alias = "Component")]
    Chemicals,
    Circuits,
    Tech,
    Consumable,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
