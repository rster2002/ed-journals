//! Includes information about engineer progress.

use crate::civilization::Engineer;
use serde::{Deserialize, Serialize};

/// Includes information about engineer progress.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum EngineerProgressEvent {
    /// Fired during startup and includes current engineer progress.
    Startup(EngineerProgressStartup),

    /// Fired when the player progresses with an engineer.
    Update(EngineerProgressUpdate),
}

/// Fired during startup and includes current engineer progress.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartup {
    /// List of current engineer progress.
    pub engineers: Vec<EngineerProgressStartupEntry>,
}

/// Entry for a single engineer and the progress.
// TODO the data for this struct is so inconsistent, it could use some work.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartupEntry {
    /// The name of the engineer.
    pub engineer: Option<String>,

    /// The id of the engineer.
    #[serde(rename = "EngineerID")]
    pub engineer_id: Option<Engineer>,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    /// The current progress unlock status.
    pub progress: Option<EngineerProgressStartupProgress>,

    /// The currently unlocked rank or highest possible 'tier' for the engineer.
    pub rank: Option<u8>,

    /// The current progress towards the next rank.
    pub rank_progress: Option<f32>,
}

/// The status for a given engineer.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EngineerProgressStartupProgress {
    /// The engineer has been unlocked and the player can apply upgrades at this engineer.
    Unlocked,

    /// The player has been invited to the engineer, but still needs to complete their unlock task.
    Invited,

    /// The engineer is known, but the player has not unlocked the engineer or has been invited.
    Known,
}

/// Fired when the player progresses with an engineer.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressUpdate {
    /// The name of the engineer.
    pub engineer: String,

    /// The id of the engineer.
    #[serde(rename = "EngineerID")]
    pub engineer_id: Engineer,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    /// The current progress unlock status.
    pub progress: Option<EngineerProgressStartupProgress>,

    /// The currently unlocked rank or highest possible 'tier' for the engineer.
    pub rank: Option<u8>,

    /// The current progress towards the next rank.
    pub rank_progress: Option<f32>,
}
