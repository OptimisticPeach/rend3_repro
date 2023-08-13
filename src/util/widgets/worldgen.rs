use std::ops::RangeInclusive;
use egui::{DragValue, Ui};
use serde::{Deserialize, Serialize};
use sphere_terrain::{AF32, DropSettings, World};
use crate::widgets::LandOpts;

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone)]
pub struct WorldGenWidget {
    pub opts: LandOpts,
    #[serde(default)]
    pub hardness_opts: LandOpts,
    pub drop_opts: DropOpts,
    #[serde(default)]
    pub wetness_evaporation: f32,
    #[serde(default)]
    pub wetness_inertia: f32,
    #[serde(default)]
    pub river_steps: usize,
    pub num_iters: usize,
    #[serde(default)]
    pub blur_iters: usize,
    #[serde(default)]
    pub blur_coeff: f32,
}

impl WorldGenWidget {
    pub fn render_on(&mut self, ui: &mut Ui, world: &mut World) -> bool {
        let mut changed = false;
        ui.collapsing(
            "Noise",
            |ui| {
                changed |= self.opts.render_on(ui, None);
            }
        );

        let mut changed_drops = false;

        ui.collapsing(
            "Drop Options",
            |ui| {
                changed_drops |= self.drop_opts.render_on(ui);
            }
        );

        let mut changed_wetness_drops = false;

        ui.label("Wetness Evaporation");
        changed_wetness_drops |= ui
            .add(egui::Slider::new(&mut self.wetness_evaporation, 0.0..=0.5))
            .changed();

        ui.label("Wetness Inertia");
        changed_wetness_drops |= ui
            .add(egui::Slider::new(&mut self.wetness_inertia, 0.0..=1.0))
            .changed();

        ui.label("River Steps");
        changed_wetness_drops |= ui
            .add(egui::Slider::new(&mut self.river_steps, 0..=100))
            .changed();

        ui.collapsing(
            "Softness Options",
            |ui| {
                changed |= self.hardness_opts.render_on(ui, Some(1.0));
            }
        );

        changed |= ui.button("Reset World").clicked();

        if changed {
            *world = World::new(self.opts.subdivisions, self.drop_opts.0);
            world.fill_noise_heights(self.opts.into());
            world.fill_hardness(self.hardness_opts.into());
        } else if changed_drops {
            world.settings = self.drop_opts.0;
        }

        ui.label("Number of drops to simulate");
        ui.add(DragValue::new(&mut self.num_iters));

        ui.label("Number of blur passes");
        ui.add(DragValue::new(&mut self.blur_iters));

        if ui.button(format!("Run {} Drops", self.num_iters)).clicked() {
            println!("Simulating {} drops", self.num_iters);
            world.simulate_node_centered_drops(self.num_iters, self.blur_iters);

            changed |= self.num_iters != 0;
        }

        ui.label("Blurred Coefficient");
        ui.add(egui::Slider::new(&mut self.blur_coeff, 0.0..=1.0))
            .changed();

        if ui.button(format!("Blur world {:.1}/{:.1}", 100.0 * self.blur_coeff, 100.0 * (1.0 - self.blur_coeff))).clicked() {
            let blurred = vec![AF32::new(0.0); world.positions.len()];
            world.blur_apply(&world.heights, &blurred);

            world
                .heights
                .iter()
                .zip(blurred.into_iter())
                .for_each(|(into, blurred)| {
                    let orig = into.load();
                    let blurred = blurred.load();
                    into.store(blurred * self.blur_coeff + orig * (1.0 - self.blur_coeff));
                });

            changed = true;
        }


        if changed_wetness_drops || changed {
            world.fill_wetness(self.wetness_evaporation, self.wetness_inertia, self.river_steps);
        }

        changed || changed_drops
    }

    pub fn get_state(&self) -> World {
        let mut world = World::new(self.opts.subdivisions, self.drop_opts.0);

        world.fill_noise_heights(self.opts.into());
        world.fill_hardness(self.hardness_opts.into());
        world.fill_wetness(self.wetness_evaporation, self.wetness_inertia, self.river_steps);

        world
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone)]
pub struct DropOpts(pub DropSettings);

impl DropOpts {
    pub fn render_on(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        let mut make = |label: &str, value: &mut f32, range: RangeInclusive<f32>| {
            ui.label(label);
            changed |= ui
                .add(egui::Slider::new(value, range))
                .changed();
        };

        make("Inertia", &mut self.0.inertia, 0.0..=1.0);
        make("Capacity", &mut self.0.capacity, 0.0..=32.0);
        make("Deposition", &mut self.0.deposition, 0.0..=1.0);
        make("Erosion", &mut self.0.erosion, 0.0..=1.0);
        make("Evaporation", &mut self.0.evaporation, 0.0..=0.5);
        make("Min Slope", &mut self.0.min_slope, 0.0..=0.06);
        make("Gravity", &mut self.0.gravity, 0.0..=15.0);

        ui.label("Max Steps");
        changed |= ui
            .add(egui::Slider::new(&mut self.0.max_steps, 0..=80))
            .changed();

        changed
    }
}
