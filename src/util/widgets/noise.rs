use sphere_terrain::noisegen::Opts;
use egui::Ui;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct LandOpts {
    /// Number of times to sample the noise
    pub octaves: usize,
    /// How smooth the noise should be (sensible values are around 0.5-1)
    pub hurst_exponent: f32,
    /// Frequency ratio between successive octaves
    pub lacunarity: f32,
    /// Maximum value
    pub max: f32,
    /// Minumum value
    pub min: f32,
    /// Scales points before sampling
    pub sample_scale: f32,
    /// Seeds the rng
    pub seed: u32,
    /// Subdivisions for land
    pub subdivisions: usize,
}

impl Default for LandOpts {
    fn default() -> Self {
        Self {
            octaves: 5,
            hurst_exponent: 0.7,
            lacunarity: 1.8,
            min: 0.8,
            max: 1.25,
            sample_scale: 0.8,
            seed: 0,
            subdivisions: 30,
        }
    }
}

impl LandOpts {
    pub fn render_on(&mut self, ui: &mut Ui, max_val: Option<f32>) -> bool {
        let mut changed = false;
        ui.label("Octaves");
        changed |= ui
            .add(egui::Slider::new(&mut self.octaves, 1..=20))
            .changed();

        ui.label("Hurst Exponent");
        changed |= ui
            .add(egui::Slider::new(&mut self.hurst_exponent, 0.4..=1.1).smart_aim(false))
            .changed();

        ui.label("Lacunarity");
        changed |= ui
            .add(egui::Slider::new(&mut self.lacunarity, 1.0..=5.0).smart_aim(false))
            .changed();

        ui.label("Min");
        changed |= ui
            .add(egui::Slider::new(&mut self.min, 0.0..=max_val.unwrap_or(2.0)).smart_aim(false))
            .changed();

        ui.label("Max");
        changed |= ui
            .add(egui::Slider::new(&mut self.max, self.min..=max_val.unwrap_or(2.0)).smart_aim(false))
            .changed();

        ui.label("Sample Scale");
        changed |= ui
            .add(egui::Slider::new(&mut self.sample_scale, 0.0..=2.0).smart_aim(false))
            .changed();

        ui.label("Seed");
        changed |= ui.add(egui::DragValue::new(&mut self.seed)).changed();

        ui.label("Subdivisions");
        changed |= ui
            .add(egui::Slider::new(&mut self.subdivisions, 0..=200))
            .changed();

        changed
    }
}

impl From<LandOpts> for Opts {
    fn from(value: LandOpts) -> Self {
        Self {
            octaves: value.octaves,
            hurst_exponent: value.hurst_exponent,
            lacunarity: value.lacunarity,
            min: value.min,
            max: value.max,
            sample_scale: value.sample_scale,
            seed: value.seed,
            offset: Vec3::ZERO,
        }
    }
}
