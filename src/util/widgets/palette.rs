use egui::Ui;
use serde::{Deserialize, Serialize};
use glam::Vec3;
use crate::color;
use crate::util::smoothstep;

type Col = [f32; 3];

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct Palette {
    snow: Col,
    tundra: Col,
    bare: Col,
    scorched: Col,

    taiga: Col,
    shrubland: Col,
    temperate_desert: Col,

    temperate_rainforest: Col,
    temperate_deciduous_forest: Col,
    grassland: Col,

    tropical_rainforest: Col,
    tropical_seasonal_forest: Col,
    subtropical_desert: Col,
}

impl Palette {
    pub fn render_on(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        egui::Grid::new("palette_grid")
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("Moisture / Elevation");
                });
                ui.horizontal_centered(|ui| {
                    ui.label("6");
                    ui.small("(wet)");
                });
                for i in (2..=5).rev() {
                    ui.label(&format!("{}", i));
                }
                ui.horizontal_centered(|ui| {
                    ui.label("1");
                    ui.small("(dry)");
                });
                ui.end_row();

                ui.horizontal_centered(|ui| {
                    ui.label("4");
                    ui.small("(high)");
                });

                for _ in 0..3 {
                    ui.horizontal_centered(|ui| {
                        ui.label("SNOW");
                        changed |= ui.color_edit_button_rgb(&mut self.snow).changed();
                    });
                }

                ui.horizontal_centered(|ui| {
                    ui.label("TUNDRA");
                    changed |= ui.color_edit_button_rgb(&mut self.tundra).changed();
                });

                ui.horizontal_centered(|ui| {
                    ui.label("BARE");
                    changed |= ui.color_edit_button_rgb(&mut self.bare).changed();
                });

                ui.horizontal_centered(|ui| {
                    ui.label("SCORCHED");
                    changed |= ui.color_edit_button_rgb(&mut self.scorched).changed();
                });
                ui.end_row();

                ui.label("3");

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("TAIGA");
                        changed |= ui.color_edit_button_rgb(&mut self.taiga).changed();
                    });
                }

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("SHURBLAND");
                        changed |= ui.color_edit_button_rgb(&mut self.shrubland).changed();
                    });
                }

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("TEMPERATE DESERT");
                        changed |= ui.color_edit_button_rgb(&mut self.temperate_desert).changed();
                    });
                }
                ui.end_row();

                ui.label("2");

                ui.horizontal_centered(|ui| {
                    ui.label("TEMPERATE RF");
                    changed |= ui.color_edit_button_rgb(&mut self.temperate_rainforest).changed();
                });

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("TEMPERATE DF");
                        changed |= ui.color_edit_button_rgb(&mut self.temperate_deciduous_forest).changed();
                    });
                }

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("GRASSLAND");
                        changed |= ui.color_edit_button_rgb(&mut self.grassland).changed();
                    });
                }

                ui.horizontal_centered(|ui| {
                    ui.label("TEMPERATE DESERT");
                    changed |= ui.color_edit_button_rgb(&mut self.temperate_desert).changed();
                });
                ui.end_row();

                ui.horizontal_centered(|ui| {
                    ui.label("1");
                    ui.small("(low)");
                });

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("TROPICAL RF");
                        changed |= ui.color_edit_button_rgb(&mut self.tropical_rainforest).changed();
                    });
                }

                for _ in 0..2 {
                    ui.horizontal_centered(|ui| {
                        ui.label("TROPICAL SF");
                        changed |= ui.color_edit_button_rgb(&mut self.tropical_seasonal_forest).changed();
                    });
                }

                ui.horizontal_centered(|ui| {
                    ui.label("GRASSLAND");
                    changed |= ui.color_edit_button_rgb(&mut self.grassland).changed();
                });

                ui.horizontal_centered(|ui| {
                    ui.label("SUBTROPICAL DESERT");
                    changed |= ui.color_edit_button_rgb(&mut self.subtropical_desert).changed();
                });
                ui.end_row();
            });

        changed
    }

    pub fn get(&self, wetness: f32, height: f32, height_scale: f32, neighbouring: bool) -> Vec3 {
        if neighbouring {
            let weight = smoothstep(height, 0.98, 1.01);
            weight * color::SALMON + (1.0 - weight) * color::DARK_GRAY
        } else {
            let palette = [
                [
                    self.subtropical_desert, self.grassland, self.tropical_seasonal_forest, self.tropical_seasonal_forest, self.tropical_rainforest, self.tropical_rainforest,
                ],
                [
                    self.temperate_desert, self.grassland, self.grassland, self.temperate_deciduous_forest, self.temperate_deciduous_forest, self.temperate_rainforest,
                ],
                [
                    self.temperate_desert, self.temperate_desert, self.shrubland, self.shrubland, self.taiga, self.taiga,
                ],
                [
                    self.scorched, self.bare, self.tundra, self.snow, self.snow, self.snow,
                ],
            ];

            let wetness_scale = wetness * 6.0;
            let height_scale = height_scale * height_scale * 4.0;

            let upper_wet = (wetness_scale.ceil() as usize).min(5);
            let lower_wet = (wetness_scale.floor() as usize).min(5);
            let upper_height = (height_scale.ceil() as usize).min(3);
            let lower_height = (height_scale.floor() as usize).min(3);

            let top_left: Vec3 = palette[upper_height][lower_wet].into();
            let top_right: Vec3 = palette[upper_height][upper_wet].into();
            let bottom_left: Vec3 = palette[lower_height][lower_wet].into();
            let bottom_right: Vec3 = palette[lower_height][upper_wet].into();

            let wet_dist = wetness_scale - wetness_scale.floor();

            let top = top_left * (1.0 - wet_dist) + top_right * wet_dist;
            let bottom = bottom_left * (1.0 - wet_dist) + bottom_right * wet_dist;

            let height_dist = height_scale - height_scale.floor();

            bottom * (1.0 - height_dist) + top * height_dist
        }
    }
}
