pub use models::destination_status::DestinationStatus;
pub use models::flags::Flags;
pub use models::flags2::Flags2;
pub use models::fuel_status::FuelStatus;
pub use models::gui_focus::GuiFocus;
pub use models::legal_status::LegalStatus;
pub use models::planet_status::PlanetStatus;
pub use models::status::OnFootStatus;
pub use models::status::ShipStatus;
pub use models::status::Status;
pub use models::status::StatusContents;
pub use models::status::StatusKind;

mod models;

pub mod blocking;

#[cfg(all(feature = "asynchronous", feature = "tokio"))]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
pub mod asynchronous;
