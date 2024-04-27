use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum EngineerProgressEvent {
    Startup(EngineerProgressStartup),
    Update(EngineerProgressUpdate),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartup {
    pub engineers: Vec<EngineerProgressStartupEntry>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartupEntry {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,

    #[serde(flatten)]
    pub progress: EngineerProgressStartupProgress,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", tag = "Progress")]
pub enum EngineerProgressStartupProgress {
    Unlocked {
        #[serde(rename = "Rank")]
        rank: u8,

        #[serde(rename = "RankProgress")]
        rank_progress: Option<f32>,
    },
    Known,
    Invited,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressUpdate {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,

    #[serde(flatten)]
    pub progress: EngineerProgressStartupProgress,
    // pub rank: Option<u8>,
    //
    // // TODO check when this is None
    // pub rank_progress: Option<f32>,
    // pub progress:
}

#[cfg(test)]
mod tests {
    use crate::models::journal_event_kind::engineer_progress_event::{
        EngineerProgressStartupEntry, EngineerProgressStartupProgress, EngineerProgressUpdate,
    };
    use serde_json::json;

    #[test]
    fn engineer_startup_entries_are_parsed_correctly() {
        let test_cases = [
            (
                EngineerProgressStartupEntry {
                    engineer: "Felicity Farseer".to_string(),
                    engineer_id: 300100,
                    progress: EngineerProgressStartupProgress::Invited,
                },
                json!({
                    "Engineer": "Felicity Farseer",
                    "EngineerID": 300100,
                    "Progress": "Invited"
                }),
            ),
            (
                EngineerProgressStartupEntry {
                    engineer: "Eleanor Bresa".to_string(),
                    engineer_id: 400011,
                    progress: EngineerProgressStartupProgress::Known,
                },
                json!({
                    "Engineer": "Eleanor Bresa",
                    "EngineerID": 400011,
                    "Progress": "Known"
                }),
            ),
            (
                EngineerProgressStartupEntry {
                    engineer: "Zacariah Nemo".to_string(),
                    engineer_id: 300050,
                    progress: EngineerProgressStartupProgress::Unlocked {
                        rank_progress: Some(0.0),
                        rank: 5,
                    },
                },
                json!({
                    "Engineer": "Zacariah Nemo",
                    "EngineerID": 300050,
                    "Progress": "Unlocked",
                    "RankProgress": 0,
                    "Rank": 5
                }),
            ),
        ];

        for (expected, value) in test_cases {
            let parsed: EngineerProgressStartupEntry = serde_json::from_value(value).unwrap();

            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn engineer_update_log_is_parsed_correctly() {
        let test = json!({
            "Engineer": "Zacariah Nemo",
            "EngineerID": 300050,
            "Rank": 4,
            "RankProgress": 0
        });

        let expected = EngineerProgressUpdate {
            engineer: "Zacariah Nemo".to_string(),
            engineer_id: 300050,
            progress: EngineerProgressStartupProgress::Unlocked {
                rank: 4,
                rank_progress: Some(0.0),
            }
        };

        let parsed: EngineerProgressUpdate = serde_json::from_value(test).unwrap();

        assert_eq!(parsed, expected);
    }
}
