use glam::Vec3;
use rand::{Rng, SeedableRng};
use rend3_types::{Handedness, Mesh, MeshBuilder};
use sphere_terrain::World;
use crate::util::widgets::palette::Palette;

fn make_colour(height: f32, wetness: f32, height_scale: f32, neighbouring: bool, palette: &Palette) -> Vec3 {
    let height_scale = 1.0 - (1.0 - height_scale).sqrt();
    let wetness = 1.0 - (1.0 - wetness).sqrt();
    palette.get(wetness, height, height_scale, neighbouring)
}

pub fn create_land_mesh(world: &World, palette: &Palette) -> Mesh {
    let scaled_positions = world
        .positions
        .iter()
        .copied()
        .zip(world.heights.iter().map(|x| x.load()))
        .map(|(x, y)| x * y)
        // .map(|(x, _)| x)
        .collect::<Vec<_>>();
    let mut inner_points = Vec::new();
    let mut inner_colours = Vec::new();

    let mut min_height = f32::INFINITY;
    let mut max_height = f32::NEG_INFINITY;

    world.heights
    // world.delta_height
        .iter()
        .for_each(|x| {
            let loaded = x.load();
            min_height = min_height.min(loaded);
            max_height = max_height.max(loaded);
        });

    let wetness_scale = get_sorted_idx(world.wetness.iter().map(|x| x.load()), |_| true, world.heights.len());
    let height_scale = get_sorted_idx(world.heights.iter().map(|x| x.load()), |x| x >= 1.0, world.heights.len());

    let mut rng = rand::rngs::StdRng::from_seed([0; 32]);

    for (source, &adj) in world.adjacent.iter().enumerate() {
        let neighbouring = adj
            .iter()
            .map(|&x| world.heights[x].load() < 1.0)
            .fold(false, |x, y| x | y) | (world.heights[source].load() < 1.0);
        let height = world.heights[source].load();
        let mut colour = make_colour(
            height,
            wetness_scale[source],
            // height_scale[source],
            (height - min_height) / (max_height - min_height),
            neighbouring,
            palette,
        );
        colour *= rng.gen_range(0.9..1.0);
        // let colour = (world.delta_height[source].load() - min_height) / (max_height - min_height);
        // println!("{}, min: {}, max: {}", colour, min_height, max_height);
        // let colour = Vec3::splat(colour);
        let colour = colour * 255.0;
        let colour = [colour.x as u8, colour.y as u8, colour.z as u8, 255];

        let mut make = |a, b, c| {
            let half = scaled_positions[source] + scaled_positions[b];
            let pt1: Vec3 = half + scaled_positions[a];
            let pt2: Vec3 = half + scaled_positions[c];

            inner_points.extend_from_slice(&[scaled_positions[source], pt1 / 3.0, pt2 / 3.0]);
            // inner_points.extend_from_slice(&[scaled_positions[source], pt1.normalize(), pt2.normalize()]);

            inner_colours.extend_from_slice(&[colour, colour, colour]);
        };

        for trio in adj.windows(3) {
            make(trio[0], trio[1], trio[2]);
        }

        let len = adj.len();
        make(adj[len - 2], adj[len - 1], adj[0]);
        make(adj[len - 1], adj[0], adj[1]);
    }

    MeshBuilder::new(inner_points, Handedness::Left)
        .with_vertex_color_0(inner_colours)
        .build()
        .unwrap()
}

fn get_sorted_idx(items: impl Iterator<Item = f32>, filter: impl Fn(f32) -> bool, len: usize) -> Vec<f32> {
    let mut ord = items.enumerate().filter(|&(_, x)| filter(x)).collect::<Vec<_>>();
    ord.sort_by(|x, y| x.1.total_cmp(&y.1));
    let mut scale = vec![0.0; len];
    let len = ord.len();
    ord
        .into_iter()
        .enumerate()
        .for_each(|(ord_idx, (actual_idx, _))| {
            let percent = ord_idx as f32 / (len - 1) as f32;
            scale[actual_idx] = percent;
        });

    scale
}
