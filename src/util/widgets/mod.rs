use crate::util::default;
use crate::util::widgets::material::PbrMaterialWidget;
use crate::util::widgets::noise::LandOpts;
// use crate::util::widgets::waves::WaterWavesWidget;
use crate::util::widgets::worldgen::WorldGenWidget;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use crate::util::widgets::palette::Palette;

pub mod material;
pub mod noise;
pub mod worldgen;
pub mod palette;

#[derive(Serialize, Deserialize, Debug)]
pub struct Widgets {
    #[serde(default)]
    pub water_material: PbrMaterialWidget,
    // #[serde(default)]
    // pub water_waves: WaterWavesWidget,
    #[serde(default)]
    pub land_material: PbrMaterialWidget,
    #[serde(default)]
    pub cloud_material: PbrMaterialWidget,
    #[serde(default)]
    pub land_options: WorldGenWidget,
    #[serde(default)]
    pub land_palette: Palette,
    pub should_save: bool,
}

impl Default for Widgets {
    fn default() -> Self {
        Self {
            water_material: default(),
            // water_waves: default(),
            land_material: default(),
            cloud_material: default(),
            land_options: default(),
            land_palette: default(),
            should_save: true,
        }
    }
}

impl Drop for Widgets {
    fn drop(&mut self) {
        if self.should_save {
            save_to_file(self);
        }
    }
}

pub fn load_from_file() -> Option<Widgets> {
    let file = std::fs::read_to_string("widget_settings").ok()?;
    Some(ron::from_str::<Widgets>(&file).ok()?)
}

pub fn save_to_file(widgets: &Widgets) {
    let saved = ron::ser::to_string_pretty(widgets, PrettyConfig::default()).unwrap();
    std::fs::write("widget_settings", saved).unwrap();
}
