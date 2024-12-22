use crate::materials::Material;
use serde::Serialize;
use std::collections::HashMap;
use crate::logs::{LogEvent, LogEventContent};
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;

#[derive(Serialize, Default)]
pub struct MaterialsStateResolver {
    pub materials: HashMap<Material, u16>,
}

impl MaterialsStateResolver {
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

impl StateResolver<LogEvent> for MaterialsStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
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
            },
            LogEventContent::MaterialCollected(event) => {
                self.add_material_count(event.name.clone(), event.count);
            },
            LogEventContent::MaterialDiscarded(event) => {
                self.remove_material_count(event.name.clone(), event.count);
            },
            LogEventContent::MaterialTrade(event) => {
                self.remove_material_count(event.paid.material.clone(), event.paid.quantity);
                self.add_material_count(event.received.material.clone(), event.received.quantity);
            },

            LogEventContent::EngineerCraft(event) => {
                for ingredient in &event.ingredients {
                    self.remove_material_count(ingredient.name.clone(), ingredient.count);
                }
            },

            _ => return FeedResult::Skipped,
        }

        FeedResult::Accepted
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::modules::state::models::resolvers::materials_state_resolver::MaterialsStateResolver;

    #[test]
    fn material_state_is_modified_correctly() {
        let mut state = MaterialsStateResolver::default();

        state.set_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 10);

        state.set_material_count(Material::Carbon, 10000);
        assert_eq!(state.get_count(&Material::Carbon), 300);

        state.remove_material_count(Material::Carbon, 20);
        assert_eq!(state.get_count(&Material::Carbon), 280);

        state.remove_material_count(Material::Carbon, 1000);
        assert_eq!(state.get_count(&Material::Carbon), 0);

        state.add_material_count(Material::Carbon, 20);
        assert_eq!(state.get_count(&Material::Carbon), 20);

        state.add_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 30);

        state.add_material_count(Material::Carbon, 1000);
        assert_eq!(state.get_count(&Material::Carbon), 300);
    }

    #[test]
    fn set_materials_without_entry_is_done_correctly() {
        let mut state = MaterialsStateResolver::default();

        state.set_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 10);

        state.set_material_count(Material::Iron, 10000);
        assert_eq!(state.get_count(&Material::Iron), 300);
    }

    #[test]
    fn add_materials_without_entry_is_done_correctly() {
        let mut state = MaterialsStateResolver::default();

        state.add_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 10);

        state.add_material_count(Material::Iron, 10000);
        assert_eq!(state.get_count(&Material::Iron), 300);
    }

    #[test]
    fn remove_materials_without_entry_is_done_correctly() {
        let mut state = MaterialsStateResolver::default();

        state.remove_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 0);
    }
}
