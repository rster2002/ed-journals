use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutEvent {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    // TODO look if this can be an enum
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,
    pub suit_mods: Vec<SuitLoadoutMod>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SuitLoadoutModule>,
}

// TODO I'm not yet sure what this should contain
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutMod {}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutModule {
    // TODO look if this can be an enum
    pub slot_name: String,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,

    // TODO look if this can be an enum
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: String,
    pub class: u8,
    pub weapon_mods: Vec<SuitLoadoutModuleWeaponMod>,
}

// TODO I'm not yet sure what this should contain
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitLoadoutModuleWeaponMod {}
