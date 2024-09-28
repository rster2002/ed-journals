use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGeologicalEntry {
    Fumarole,
    FumaroleAmmoniaGeysers,
    FumaroleCarbondioxideGeysers,
    FumaroleHeliumGeysers,
    FumaroleMethaneGeysers,
    FumaroleNitrogenGeysers,
    FumaroleSilicateVapourGeysers,
    FumaroleSulfurDioxideMagma,
    FumaroleWaterGeysers,

    IceFumarole,
    IceFumaroleAmmoniaGeysers,
    IceFumaroleCarbonDioxideGeysers,
    IceFumaroleHeliumGeysers,
    IceFumaroleMethaneGeysers,
    IceFumaroleNitrogenGeysers,
    IceFumaroleSilicateVapourGeysers,
    IceFumaroleSulfurDioxideMagma,
    IceFumaroleWaterGeysers,

    GasVents,
    GasVentsAmmoniaGeysers,
    GasVentsCarbonDioxideGeysers,
    GasVentsHeliumGeysers,
    GasVentsMethaneGeysers,
    GasVentsNitrogenGeysers,
    GasVentsSilicateVapourGeysers,
    GasVentsSulfurDioxideMagma,
    GasVentsWaterGeysers,

    Geysers,
    GeysersAmmoniaGeysers,
    GeysersCarbonDioxideGeysers,
    GeysersHeliumGeysers,
    GeysersMethaneGeysers,
    GeysersNitrogenGeysers,
    GeysersSulfurDioxideMagma,
    GeysersWaterGeysers,

    IceGeysers,
    IceGeysersAmmoniaGeysers,
    IceGeysersCarbonDioxideGeysers,
    IceGeysersHeliumGeysers,
    IceGeysersMethaneGeysers,
    IceGeysersNitrogenGeysers,
    IceGeysersSulfurDioxideMagma,
    IceGeysersWaterGeysers,

    LavaSpouts,
    LavaSpoutsIronMagma,
    LavaSpoutsSilicateMagma,
    LavaSpoutsSulfurDioxideMagma,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexGeologicalEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexGeologicalEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexGeologicalError {
    #[error("Failed to parse geological codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown geological codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexGeologicalEntry {
    type Err = CodexGeologicalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(CodexGeologicalError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "fumarole" => CodexGeologicalEntry::Fumarole,
            "fumarole_ammoniageysers" => CodexGeologicalEntry::FumaroleAmmoniaGeysers,
            "fumarole_carbondioxidegeysers" => CodexGeologicalEntry::FumaroleCarbondioxideGeysers,
            "fumarole_heliumgeysers" => CodexGeologicalEntry::FumaroleHeliumGeysers,
            "fumarole_methanegeysers" => CodexGeologicalEntry::FumaroleMethaneGeysers,
            "fumarole_nitrogengeysers" => CodexGeologicalEntry::FumaroleNitrogenGeysers,
            "fumarole_silicatevapourgeysers" => CodexGeologicalEntry::FumaroleSilicateVapourGeysers,
            "fumarole_sulphurdioxidemagma" => CodexGeologicalEntry::FumaroleSulfurDioxideMagma,
            "fumarole_watergeysers" => CodexGeologicalEntry::FumaroleWaterGeysers,

            "icefumarole" => CodexGeologicalEntry::IceFumarole,
            "icefumarole_ammoniageysers" => CodexGeologicalEntry::IceFumaroleAmmoniaGeysers,
            "icefumarole_carbondioxidegeysers" => CodexGeologicalEntry::IceFumaroleCarbonDioxideGeysers,
            "icefumarole_heliumgeysers" => CodexGeologicalEntry::IceFumaroleHeliumGeysers,
            "icefumarole_methanegeysers" => CodexGeologicalEntry::IceFumaroleMethaneGeysers,
            "icefumarole_nitrogengeysers" => CodexGeologicalEntry::IceFumaroleNitrogenGeysers,
            "icefumarole_silicatevapourgeysers" => CodexGeologicalEntry::IceFumaroleSilicateVapourGeysers,
            "icefumarole_sulphurdioxidemagma" => CodexGeologicalEntry::IceFumaroleSulfurDioxideMagma,
            "icefumarole_watergeysers" => CodexGeologicalEntry::IceFumaroleWaterGeysers,

            "gas_vents" => CodexGeologicalEntry::GasVents,
            "gas_vents_ammoniageysers" => CodexGeologicalEntry::GasVentsAmmoniaGeysers,
            "gas_vents_carbondioxidegeysers" => CodexGeologicalEntry::GasVentsCarbonDioxideGeysers,
            "gas_vents_heliumgeysers" => CodexGeologicalEntry::GasVentsHeliumGeysers,
            "gas_vents_methanegeysers" => CodexGeologicalEntry::GasVentsMethaneGeysers,
            "gas_vents_nitrogengeysers" => CodexGeologicalEntry::GasVentsNitrogenGeysers,
            "gas_vents_silicatevapourgeysers" => CodexGeologicalEntry::GasVentsSilicateVapourGeysers,
            "gas_vents_sulphurdioxidemagma" => CodexGeologicalEntry::GasVentsSulfurDioxideMagma,
            "gas_vents_watergeysers" => CodexGeologicalEntry::GasVentsWaterGeysers,

            "geysers" => CodexGeologicalEntry::Geysers,
            "geysers_ammoniageysers" => CodexGeologicalEntry::GeysersAmmoniaGeysers,
            "geysers_carbondioxidegeysers" => CodexGeologicalEntry::GeysersCarbonDioxideGeysers,
            "geysers_heliumgeysers" => CodexGeologicalEntry::GeysersHeliumGeysers,
            "geysers_methanegeysers" => CodexGeologicalEntry::GeysersMethaneGeysers,
            "geysers_nitrogengeysers" => CodexGeologicalEntry::GeysersNitrogenGeysers,
            "geysers_sulphurdioxidemagma" => CodexGeologicalEntry::GeysersSulfurDioxideMagma,
            "geysers_watergeysers" => CodexGeologicalEntry::GeysersWaterGeysers,

            "icegeysers" => CodexGeologicalEntry::IceGeysers,
            "icegeysers_ammoniageysers" => CodexGeologicalEntry::IceGeysersAmmoniaGeysers,
            "icegeysers_carbondioxidegeysers" => CodexGeologicalEntry::IceGeysersCarbonDioxideGeysers,
            "icegeysers_heliumgeysers" => CodexGeologicalEntry::IceGeysersHeliumGeysers,
            "icegeysers_methanegeysers" => CodexGeologicalEntry::IceGeysersMethaneGeysers,
            "icegeysers_nitrogengeysers" => CodexGeologicalEntry::IceGeysersNitrogenGeysers,
            "icegeysers_sulphurdioxidemagma" => CodexGeologicalEntry::IceGeysersSulfurDioxideMagma,
            "icegeysers_watergeysers" => CodexGeologicalEntry::IceGeysersWaterGeysers,

            "lava_spouts" => CodexGeologicalEntry::LavaSpouts,
            "lava_spouts_ironmagma" => CodexGeologicalEntry::LavaSpoutsIronMagma,
            "lava_spouts_silicatemagma" => CodexGeologicalEntry::LavaSpoutsSilicateMagma,
            "lava_spouts_sulphurdioxidemagma" => CodexGeologicalEntry::LavaSpoutsSulfurDioxideMagma,

            #[cfg(feature = "allow-unknown")]
            _ => CodexGeologicalEntry::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(CodexGeologicalError::UnknownEntry(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(CodexGeologicalEntry);

impl Display for CodexGeologicalEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            // CodexGeologicalEntry::Fumarole => "Fumarole",
            // CodexGeologicalEntry::IceFumarole => "Ice Fumarole",
            // CodexGeologicalEntry::Geyser => "Geyser",
            // CodexGeologicalEntry::IceGeyser => "Ice Geyser",
            // CodexGeologicalEntry::LavaSpout => "Lava Spout",
            // CodexGeologicalEntry::GasVent => "Gas Vent",
            CodexGeologicalEntry::Fumarole => "Fumarole",
            CodexGeologicalEntry::FumaroleAmmoniaGeysers => "Ammonia Fumarole",
            CodexGeologicalEntry::FumaroleCarbondioxideGeysers => "Carbon Dioxide Fumarole",
            CodexGeologicalEntry::FumaroleHeliumGeysers => "Helium Fumarole",
            CodexGeologicalEntry::FumaroleMethaneGeysers => "Methane Fumarole",
            CodexGeologicalEntry::FumaroleNitrogenGeysers => "Nitrogen Fumarole",
            CodexGeologicalEntry::FumaroleSilicateVapourGeysers => "Silicate Vapour Fumarole",
            CodexGeologicalEntry::FumaroleSulfurDioxideMagma => "Sulfur Dioxide Fumarole",
            CodexGeologicalEntry::FumaroleWaterGeysers => "Water Geysers Fumarole",

            CodexGeologicalEntry::IceFumarole => "Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleAmmoniaGeysers => "Ammonia Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleCarbonDioxideGeysers => "Carbon Dioxide Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleHeliumGeysers => "Helium Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleMethaneGeysers => "Methane Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleNitrogenGeysers => "Nitrogen Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleSilicateVapourGeysers => "Silicate Vapour Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleSulfurDioxideMagma => "Sulfur Dioxide Ice Fumarole",
            CodexGeologicalEntry::IceFumaroleWaterGeysers => "Water Geysers Ice Fumarole",

            CodexGeologicalEntry::GasVents => "Gas Vents",
            CodexGeologicalEntry::GasVentsAmmoniaGeysers => "Ammonia Gas Vents",
            CodexGeologicalEntry::GasVentsCarbonDioxideGeysers => "Carbon Dioxide Gas Vents",
            CodexGeologicalEntry::GasVentsHeliumGeysers => "Helium Gas Vents",
            CodexGeologicalEntry::GasVentsMethaneGeysers => "Methane Gas Vents",
            CodexGeologicalEntry::GasVentsNitrogenGeysers => "Nitrogen Gas Vents",
            CodexGeologicalEntry::GasVentsSilicateVapourGeysers => "Silicate Vapour Gas Vents",
            CodexGeologicalEntry::GasVentsSulfurDioxideMagma => "Sulfur Dioxide Gas Vents",
            CodexGeologicalEntry::GasVentsWaterGeysers => "Water Gas Vents",

            CodexGeologicalEntry::Geysers => "Geysers",
            CodexGeologicalEntry::GeysersAmmoniaGeysers => "Ammonia Geysers",
            CodexGeologicalEntry::GeysersCarbonDioxideGeysers => "Carbon Dioxide Geysers",
            CodexGeologicalEntry::GeysersHeliumGeysers => "Helium Geysers",
            CodexGeologicalEntry::GeysersMethaneGeysers => "Methane Geysers",
            CodexGeologicalEntry::GeysersNitrogenGeysers => "Nitrogen Geysers",
            CodexGeologicalEntry::GeysersSulfurDioxideMagma => "Sulfur Dioxide Geysers",
            CodexGeologicalEntry::GeysersWaterGeysers => "Water Geysers",

            CodexGeologicalEntry::IceGeysers => "Ice Geysers",
            CodexGeologicalEntry::IceGeysersAmmoniaGeysers => "Ammonia Ice Geysers",
            CodexGeologicalEntry::IceGeysersCarbonDioxideGeysers => "Carbon Dioxide Ice Geysers",
            CodexGeologicalEntry::IceGeysersHeliumGeysers => "Helium Ice Geysers",
            CodexGeologicalEntry::IceGeysersMethaneGeysers => "Methane Ice Geysers",
            CodexGeologicalEntry::IceGeysersNitrogenGeysers => "Nitrogen Ice Geysers",
            CodexGeologicalEntry::IceGeysersSulfurDioxideMagma => "Sulfur Dioxide Ice Geysers",
            CodexGeologicalEntry::IceGeysersWaterGeysers => "Water Ice Geysers",

            CodexGeologicalEntry::LavaSpouts => "Lava Spouts",
            CodexGeologicalEntry::LavaSpoutsIronMagma => "Iron Magma Lava Spouts",
            CodexGeologicalEntry::LavaSpoutsSilicateMagma => "Silicate Magma Lava Spouts",
            CodexGeologicalEntry::LavaSpoutsSulfurDioxideMagma => "Sulfur Dioxide Magma Lava Spouts",

            #[cfg(feature = "allow-unknown")]
            CodexGeologicalEntry::Unknown(unknown) => return write!(f, "Unknown geological codex entry: {}", unknown),
        })
    }
}
