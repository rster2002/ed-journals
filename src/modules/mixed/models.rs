#[cfg(all(feature = "models-trading", feature = "models-odyssey"))]
pub mod mixed_commodity;

#[cfg(all(feature = "models-materials", feature = "models-odyssey"))]
pub mod mixed_material;

#[cfg(all(feature = "models-materials", feature = "models-odyssey"))]
pub mod mixed_material_category;
