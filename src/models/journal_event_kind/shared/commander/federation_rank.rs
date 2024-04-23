use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FederationRank {
    None,
    Recruit,
    Cadet,
    Midshipman,
    PettyOfficer,
    ChiefPettyOfficer,
    WarrantOfficer,
    Ensign,
    Lieutenant,
    LieutenantCommander,
    PostCommander,
    PostCaptain,
    RearAdmiral,
    ViceAdmiral,
    Admiral,
}
