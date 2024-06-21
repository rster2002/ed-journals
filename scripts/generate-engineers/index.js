const file = Bun.file("./input.csv");

const text = await file.text();
const entries = text.split("\n")
    .filter(line => line !== "")
    .map(line => {
        let parts = line.split(",");

        return {
            id: parts[0],
            system_address: parts[1],
            market_id: parts[2],
            key: parts[3],
            name: parts[4],
        };
    });

Bun.write("output.rs", `use thiserror::Error;
use crate::from_str_deserialize_impl;

enum Engineer {
    ${entries.map(i => i.key).join(",\n\t")},
}

impl Engineer {
    pub fn get_system_address(&self) -> u64 {
        match self {
            ${entries.map(i => `Engineer::${i.key} => ${i.system_address}`).join(",\n\t\t\t")},
        }
    }
    
    pub fn get_market_id(&self) -> u64 {
        match self {
            ${entries.map(i => `Engineer::${i.key} => ${i.market_id}`).join(",\n\t\t\t")},
        }
    }
}

impl Display for Engineer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ${entries.map(i => `Engineer::${i.key} => "${i.name}"`).join(",\n\t\t\t\t")},
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum EngineerError {
    #[error("Unknown engineer for id: {0}")]
    UnknownEngineerId(u64),
}

impl TryFrom<u64> for Engineer {
    type Error = EngineerError;
    
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            ${entries.map(i => `${i.id} => Engineer::${i.key}`).join(",\n\t\t\t")},
        
            _ => return Err(EngineerError::UnknownEngineerId(value)),
        })
    }
}

from_str_deserialize_impl!(Engineer);
`)
