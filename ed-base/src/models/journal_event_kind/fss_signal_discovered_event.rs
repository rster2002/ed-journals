use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSSignalDiscoveredEvent {
    pub system_address: u64,
    pub signal_name: String,

    #[serde(rename = "SignalName_Localised")]
    pub signal_name_localised: Option<String>,

    #[serde(default)]
    pub is_station: bool,
}

mod tests {
    fn fss_signal_discovered_event_is_parsed_correctly() {
        let _test_cases = [
            r#"
                {
                    "timestamp": "2022-11-30T20:13:44Z",
                    "event": "FSSSignalDiscovered",
                    "SystemAddress": 671759803809,
                    "SignalName": "Golden Falcon"
                }
            "#,
            r#"
                {
                    "timestamp": "2022-11-30T20:13:44Z",
                    "event": "FSSSignalDiscovered",
                    "SystemAddress": 671759803809,
                    "SignalName": "$MULTIPLAYER_SCENARIO77_TITLE;",
                    "SignalName_Localised": "Resource Extraction Site [Low]"
                }
            "#,
            r#"
                {
                    "timestamp": "2022-11-30T20:13:44Z",
                    "event": "FSSSignalDiscovered",
                    "SystemAddress": 671759803809,
                    "SignalName": "DECIPULA XFY-23L",
                    "IsStation": true
                }
            "#,
        ];
    }
}
