use crate::exploration::CodexPlanetEntry;

pub fn format_codex_string(entry: &str) -> String {
    format!("Codex_Ent_{}_Name;", entry)
}