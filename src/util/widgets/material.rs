use egui::Ui;
use glam::{Vec3, Vec4};
use rend3_routine::pbr::{MaterialComponent, PbrMaterial};
use rend3_types::Texture2DHandle;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PbrMaterialWidget {
    #[serde(default)]
    metallic_factor: f32,
    #[serde(default)]
    metallic_enable: bool,
    #[serde(default)]
    metallic: Option<f32>,
    #[serde(default)]
    roughness_factor: f32,
    #[serde(default)]
    roughness_enable: bool,
    #[serde(default)]
    roughness: Option<f32>,
    #[serde(default)]
    clearcoat_factor: f32,
    #[serde(default)]
    clearcoat_enable: bool,
    #[serde(default)]
    clearcoat: Option<f32>,
    #[serde(default)]
    clearcoat_roughness_factor: f32,
    #[serde(default)]
    clearcoat_roughness_enable: bool,
    #[serde(default)]
    clearcoat_roughness: Option<f32>,
    #[serde(default)]
    reflectance_factor: f32,
    #[serde(default)]
    reflectance_enable: bool,
    #[serde(default)]
    #[serde(with = "RefMaterialComponent")]
    reflectance: MaterialComponent<f32>,
    #[serde(default)]
    emissive_enable: bool,
    #[serde(default)]
    emissive_colour: [f32; 3],
    #[serde(default)]
    albedo_colour: [f32; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(remote = "MaterialComponent")]
pub enum RefMaterialComponent<T> {
    None,
    Value(T),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    Texture(Texture2DHandle),
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    TextureValue {
        texture: Texture2DHandle,
        value: T,
    },
}

impl<T> Default for RefMaterialComponent<T> {
    fn default() -> Self {
        RefMaterialComponent::None
    }
}

impl Default for PbrMaterialWidget {
    fn default() -> Self {
        PbrMaterialWidget {
            metallic_factor: 0.0,
            metallic_enable: false,
            metallic: None,
            roughness_factor: 0.0,
            roughness_enable: false,
            roughness: None,
            clearcoat_factor: 0.0,
            clearcoat_enable: false,
            clearcoat: None,
            clearcoat_roughness_factor: 0.0,
            clearcoat_roughness_enable: false,
            clearcoat_roughness: None,
            reflectance_factor: 0.0,
            reflectance_enable: false,
            reflectance: MaterialComponent::None,
            emissive_enable: false,
            emissive_colour: [0.0f32; 3],
            albedo_colour: [0.0f32; 4],
        }
    }
}

impl PbrMaterialWidget {
    pub fn render_on(&mut self, ui: &mut Ui, vertex: bool) -> Option<PbrMaterial> {
        let mut changed;

        ui.label("Albedo");
        changed = ui
            .color_edit_button_rgba_unmultiplied(&mut self.albedo_colour)
            .changed();

        fn make_ui<T>(
            ui: &mut Ui,
            name: &'static str,
            enable: &mut bool,
            factor: &mut f32,
            none: T,
            make: fn(f32) -> T,
            place: &mut T,
            changed: &mut bool,
        ) {
            ui.label(name);
            ui.group(|ui| {
                ui.set_enabled(*enable);
                if ui
                    .add(egui::Slider::new(factor, 0.0..=1.0).text("factor"))
                    .changed
                {
                    *changed = true;
                    *place = make(*factor);
                }
            });
            if ui.checkbox(enable, "enable").changed {
                if *enable {
                    *place = make(*factor);
                } else {
                    *place = none;
                }
                *changed = true;
            }
        }

        make_ui(
            ui,
            "Metallic",
            &mut self.metallic_enable,
            &mut self.metallic_factor,
            None,
            Some,
            &mut self.metallic,
            &mut changed,
        );
        make_ui(
            ui,
            "Roughness",
            &mut self.roughness_enable,
            &mut self.roughness_factor,
            None,
            Some,
            &mut self.roughness,
            &mut changed,
        );
        make_ui(
            ui,
            "Clearcoat",
            &mut self.clearcoat_enable,
            &mut self.clearcoat_factor,
            None,
            Some,
            &mut self.clearcoat,
            &mut changed,
        );
        make_ui(
            ui,
            "Clearcoat Roughness",
            &mut self.clearcoat_roughness_enable,
            &mut self.clearcoat_roughness_factor,
            None,
            Some,
            &mut self.clearcoat_roughness,
            &mut changed,
        );
        make_ui(
            ui,
            "Reflectance",
            &mut self.reflectance_enable,
            &mut self.reflectance_factor,
            MaterialComponent::None,
            MaterialComponent::Value,
            &mut self.reflectance,
            &mut changed,
        );
        ui.label("Emissive");
        ui.group(|ui| {
            ui.set_enabled(self.emissive_enable);
            if ui
                .color_edit_button_rgb(&mut self.emissive_colour)
                .changed()
            {
                changed = true;
            }
        });
        if ui.checkbox(&mut self.emissive_enable, "enable").changed {
            changed = true;
        }

        if changed {
            Some(self.get_state(vertex))
        } else {
            None
        }
    }
    pub fn get_state(&self, vertex: bool) -> PbrMaterial {
        PbrMaterial {
            albedo: if vertex {
                rend3_routine::pbr::AlbedoComponent::ValueVertex {
                    value: Vec4::from_array(self.albedo_colour),
                    srgb: false,
                }
            } else {
                rend3_routine::pbr::AlbedoComponent::Value(Vec4::from_array(self.albedo_colour))
            },

            transparency: if vertex {
                rend3_routine::pbr::Transparency::Opaque
            } else {
                rend3_routine::pbr::Transparency::Blend
            },
            metallic_factor: self.metallic,
            roughness_factor: self.roughness,
            clearcoat_factor: self.clearcoat,
            clearcoat_roughness_factor: self.clearcoat_roughness,
            reflectance: self.reflectance.clone(),
            emissive: if self.emissive_enable {
                MaterialComponent::Value(Vec3::from_array(self.emissive_colour))
            } else {
                MaterialComponent::None
            },
            ..PbrMaterial::default()
        }
    }
}
