use std::collections::HashMap;
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::materials::Material;
use crate::state::{EventSink, SinkResult};

#[derive(Debug, Clone, Default)]
pub struct MaterialsState {
    pub materials: HashMap<Material, u16>,
}

impl MaterialsState {
    pub fn set_material_count(&mut self, material: Material, count: u16) {
        let count = count.min(material.grade().grade_storage_limit());

        self.materials
            .entry(material)
            .and_modify(|current| *current = count)
            .or_insert(count);
    }

    pub fn add_material_count(&mut self, material: Material, count: u16) {
        self.materials
            .entry(material.clone())
            .and_modify(|current| {
                *current = (*current + count).min(material.grade().grade_storage_limit())
            })
            .or_insert(count.min(material.grade().grade_storage_limit()));
    }

    pub fn remove_material_count(&mut self, materials: Material, count: u16) {
        self.materials
            .entry(materials.clone())
            .and_modify(|current| *current = current.saturating_sub(count))
            .or_insert(0);
    }

    pub fn get_count(&self, material: &Material) -> u16 {
        let Some(count) = self.materials.get(material) else {
            return 0;
        };

        *count
    }

    pub fn is_full(&self, material: &Material) -> bool {
        let Some(count) = self.materials.get(material) else {
            return false;
        };

        count >= &material.grade().grade_storage_limit()
    }
}

impl EventSink for MaterialsState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        match &log_event.content {
            LogEventContent::Materials(event) => {
                for material in &event.raw {
                    self.set_material_count(material.name.clone(), material.count);
                }

                for material in &event.encoded {
                    self.set_material_count(material.name.clone(), material.count);
                }

                for material in &event.manufactured {
                    self.set_material_count(material.name.clone(), material.count);
                }
            }
            LogEventContent::MaterialCollected(event) => {
                self.add_material_count(event.name.clone(), event.count);
            }
            LogEventContent::MaterialDiscarded(event) => {
                self.remove_material_count(event.name.clone(), event.count);
            }
            LogEventContent::MaterialTrade(event) => {
                self.remove_material_count(event.paid.material.clone(), event.paid.quantity);
                self.add_material_count(event.received.material.clone(), event.received.quantity);
            }

            LogEventContent::EngineerCraft(event) => {
                for ingredient in &event.ingredients {
                    self.remove_material_count(ingredient.name.clone(), ingredient.count);
                }
            }

            _ => return SinkResult::Ignored,
        }

        SinkResult::Accepted
    }
}