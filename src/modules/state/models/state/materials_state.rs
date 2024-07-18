use std::collections::HashMap;
use serde::Serialize;
use crate::materials::Material;

#[derive(Serialize, Default)]
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
            .and_modify(|current| *current = (*current + count).min(material.grade().grade_storage_limit()))
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

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::state::models::state::materials_state::MaterialsState;

    #[test]
    fn material_state_is_modified_correctly() {
        let mut state = MaterialsState::default();

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
        let mut state = MaterialsState::default();

        state.set_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 10);

        state.set_material_count(Material::Iron, 10000);
        assert_eq!(state.get_count(&Material::Iron), 300);
    }

    #[test]
    fn add_materials_without_entry_is_done_correctly() {
        let mut state = MaterialsState::default();

        state.add_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 10);

        state.add_material_count(Material::Iron, 10000);
        assert_eq!(state.get_count(&Material::Iron), 300);
    }

    #[test]
    fn remove_materials_without_entry_is_done_correctly() {
        let mut state = MaterialsState::default();

        state.remove_material_count(Material::Carbon, 10);
        assert_eq!(state.get_count(&Material::Carbon), 0);
    }
}
