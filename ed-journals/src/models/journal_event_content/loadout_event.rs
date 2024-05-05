use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub ship_name: String,
    pub ship_ident: String,
    pub modules: Vec<LoadoutModule>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutModule {
    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: u8,
    pub health: f32,

    // TODO check when this value is used
    pub value: Option<u32>,
    pub ammo_in_clip: Option<u32>,
}

#[cfg(test)]
mod tests {
    use crate::models::journal_event_content::loadout_event::LoadoutEvent;
    use serde_json::json;

    #[test]
    fn loadout_event_is_parsed_correctly() {
        let value = serde_json::from_value::<LoadoutEvent>(json!({
          "Ship": "CobraMkIII",
          "ShipID": 1,
          "ShipName": "Flat Head",
          "ShipIdent": "UNSC-1",
          "Modules": [
            {
              "Slot": "Armour",
              "Item": "CobraMkIII_Armour_Grade1",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 0
            },
            {
              "Slot": "PowerPlant",
              "Item": "Int_Powerplant_Size4_Class2",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 59633
            },
            {
              "Slot": "MainEngines",
              "Item": "Int_Engine_Size4_Class2",
              "On": true,
              "Priority": 0,
              "Health": 1.000000,
              "Value": 59633
            },
            {
              "Slot": "FrameShiftDrive",
              "Item": "Int_Hyperdrive_Size4_Class2",
              "On": true,
              "Priority": 0,
              "Health": 1.000000,
              "Value": 59633
            },
            {
              "Slot": "LifeSupport",
              "Item": "Int_LifeSupport_Size3_Class2",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 10133
            },
            {
              "Slot": "PowerDistributor",
              "Item": "Int_PowerDistributor_Size3_Class2",
              "On": true,
              "Priority": 0,
              "Health": 1.000000,
              "Value": 10133
            },
            {
              "Slot": "Radar",
              "Item": "Int_Sensors_Size3_Class2",
              "On": true,
              "Priority": 0,
              "Health": 1.000000,
              "Value": 10133
            },
            {
              "Slot": "FuelTank",
              "Item": "Int_FuelTank_Size4_Class3",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 24734
            },
            {
              "Slot": "Slot01_Size4",
              "Item": "Int_CargoRack_Size4_Class1",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 34328
            },
            {
              "Slot": "Slot02_Size4",
              "Item": "Int_CargoRack_Size4_Class1",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 34328
            },
            {
              "Slot": "Slot03_Size4",
              "Item": "Int_ShieldGenerator_Size4_Class2",
              "On": true,
              "Priority": 0,
              "Health": 1.000000,
              "Value": 59633
            },
            {
              "Slot": "Slot04_Size2",
              "Item": "Int_FuelScoop_Size2_Class5",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 284844
            },
            {
              "Slot": "Slot05_Size2",
              "Item": "Int_StellarBodyDiscoveryScanner_Standard",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 1000
            },
            {
              "Slot": "Slot06_Size2",
              "Item": "Int_Repairer_Size2_Class3",
              "On": true,
              "Priority": 1,
              "AmmoInClip": 2300,
              "Health": 1.000000,
              "Value": 162000
            },
            {
              "Slot": "PlanetaryApproachSuite",
              "Item": "Int_PlanetApproachSuite",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 500
            },
            {
              "Slot": "ShipCockpit",
              "Item": "CobraMkIII_Cockpit",
              "On": true,
              "Priority": 1,
              "Health": 1.000000,
              "Value": 0
            },
            {
              "Slot": "CargoHatch",
              "Item": "ModularCargoBayDoor",
              "On": true,
              "Priority": 2,
              "Health": 1.000000,
              "Value": 0
            }
          ]
        }));

        assert!(value.is_ok());
    }
}
