#![feature(portable_simd)]

use crate::util::input_manager::InputManager;
use crate::util::widgets;

use glam::{Mat4, Vec3, Vec4};
use rust_embed::RustEmbed;

use crate::util::camera::Camera;
use rend3::RendererProfile;
use std::sync::Arc;
use rend3_routine::base::BaseRenderGraph;

pub mod color;
mod geometry;
pub mod util;

fn main() {
    // Setup logging
    env_logger::init();

    // Create event loop and window
    let event_loop = winit::event_loop::EventLoop::new();
    let window = {
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("rend3 cube");
        builder.build(&event_loop).expect("Could not build window")
    };

    let window_size = window.inner_size();

    // Create the Instance, Adapter, and Device. We can specify preferred backend,
    // device name, or rendering profile. In this case we let rend3 choose for us.
    let iad = pollster::block_on(rend3::create_iad(
        None,
        None,
        Some(RendererProfile::CpuDriven),
        None,
    ))
    .unwrap();

    // The one line of unsafe needed. We just need to guarentee that the window
    // outlives the use of the surface.
    //
    // SAFETY: this surface _must_ not be used after the `window` dies. Both the
    // event loop and the renderer are owned by the `run` closure passed to winit,
    // so rendering work will stop after the window dies.
    let surface = Arc::new(unsafe { iad.instance.create_surface(&window) }.unwrap());
    // Get the preferred format for the surface.
    let caps = surface.get_capabilities(&iad.adapter);
    let preferred_format = caps.formats[0];

    // Configure the surface to be ready for rendering.
    rend3::configure_surface(
        &surface,
        &iad.device,
        preferred_format,
        glam::UVec2::new(window_size.width, window_size.height),
        rend3::types::PresentMode::Fifo,
    );

    // Make us a renderer.
    let renderer = rend3::Renderer::new(
        iad,
        rend3::types::Handedness::Left,
        Some(window_size.width as f32 / window_size.height as f32),
    )
    .unwrap();

    // Create the shader preprocessor with all the default shaders added.
    let mut spp = rend3::ShaderPreProcessor::new();
    rend3_routine::builtin_shaders(&mut spp);

    // Create the base rendergraph.
    let base_rendergraph = BaseRenderGraph::new(&renderer, &spp);

    let mut data_core = renderer.data_core.lock();
    let pbr_routine = rend3_routine::pbr::PbrRoutine::new(
        &renderer,
        &mut data_core,
        &spp,
        &base_rendergraph.interfaces,
    );
    drop(data_core);

    let tonemapping_routine = rend3_routine::tonemapping::TonemappingRoutine::new(
        &renderer,
        &spp,
        &base_rendergraph.interfaces,
        preferred_format,
    );

    let mut widgets = widgets::load_from_file().unwrap_or_default();
    let mut world = widgets.land_options.get_state();

    println!("worldlen: {}", world.positions.len());

    // Create mesh and calculate smooth normals based on vertices
    let land_mesh = geometry::land::create_land_mesh(&world, &widgets.land_palette);

    // Add mesh to renderer's world.
    //
    // All handles are refcounted, so we only need to hang onto the handle until we
    // make an object.
    let mut land_mesh_handle = renderer.add_mesh(land_mesh);

    // Add PBR material with all defaults except a single color.
    let land_material = renderer.add_material(widgets.land_material.get_state(true));

    // Combine the mesh and the material with a location to give an object.

    let mut land_object = rend3::types::Object {
        mesh_kind: rend3::types::ObjectMeshKind::Static(land_mesh_handle.clone()),
        material: land_material.clone(),
        transform: Mat4::from_scale(Vec3::splat(6.0)),
    };

    // Creating an object will hold onto both the mesh and the material
    // even if they are deleted.
    // let water = renderer.add_object(water_object);
    let mut land = renderer.add_object(land_object.clone());

    // Set camera's location
    renderer.set_camera_data(rend3::types::Camera {
        projection: rend3::types::CameraProjection::Perspective {
            vfov: 60.0,
            near: 0.1,
        },
        view: Mat4::IDENTITY,
    });

    // Create a single directional light
    //
    // We need to keep the directional light handle alive.

    let _directional_handles = vec![
        renderer.add_directional_light(rend3::types::DirectionalLight {
            color: color::ALICE_BLUE,
            intensity: 10.0,
            // Direction will be normalized
            direction: Vec3::new(2.0, -0.0, 1.0),
            distance: 0.0,
            resolution: 4,
        }),
        renderer.add_directional_light(rend3::types::DirectionalLight {
            color: color::ALICE_BLUE,
            intensity: 10.0,
            // Direction will be normalized
            direction: Vec3::new(-2.0, -0.0, 1.0),
            distance: 0.0,
            resolution: 4,
        }),
        renderer.add_directional_light(rend3::types::DirectionalLight {
            color: color::ALICE_BLUE,
            intensity: 10.0,
            // Direction will be normalized
            direction: Vec3::new(0.0, -0.0, -1.0),
            distance: 0.0,
            resolution: 4,
        }),
    ];

    let mut egui_routine = rend3_egui::EguiRenderRoutine::new(
        &*renderer,
        preferred_format,
        rend3_types::SampleCount::One,
        window_size.width,
        window_size.height,
        window.scale_factor() as f32,
    );

    let context = egui::Context::default();
    let mut platform = egui_winit::State::new(&event_loop);
    platform.set_pixels_per_point(window.scale_factor() as f32);

    let mut resolution = glam::UVec2::new(window_size.width, window_size.height);

    let mut input_manager = InputManager::new();
    let mut camera = Camera::default();
    let mut time = std::time::Instant::now();

    event_loop.run(move |event, _, control| {
        match event {
            winit::event::Event::WindowEvent { event, .. }
                if platform.on_event(&context, &event).consumed =>
            {
                return
            }

            // Close button was clicked, we should close.
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control = winit::event_loop::ControlFlow::Exit;
            }
            // Window was resized, need to resize renderer.
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(physical_size),
                ..
            } => {
                egui_routine.resize(
                    physical_size.width,
                    physical_size.height,
                    window.scale_factor() as f32,
                );
                resolution = glam::UVec2::new(physical_size.width, physical_size.height);
                println!("new resolution: {:?}", resolution);
                // Reconfigure the surface for the new size.
                rend3::configure_surface(
                    &surface,
                    &renderer.device,
                    preferred_format,
                    resolution,
                    rend3::types::PresentMode::Fifo,
                );
                // Tell the renderer about the new aspect ratio.
                renderer.set_aspect_ratio(resolution.x as f32 / resolution.y as f32);
            }
            // Render!
            winit::event::Event::MainEventsCleared => {
                // egui stuff
                context.begin_frame(platform.take_egui_input(&window));

                egui::Window::new("land settings")
                    .resizable(true)
                    .default_open(true)
                    .show(&context, |ui| {
                        if widgets.land_options.render_on(ui, &mut world) {
                            let mesh = geometry::land::create_land_mesh(&world, &widgets.land_palette);
                            land_mesh_handle = renderer.add_mesh(mesh);
                            land_object.mesh_kind =
                                rend3::types::ObjectMeshKind::Static(land_mesh_handle.clone());
                            land = renderer.add_object(land_object.clone());
                        }
                    });

                let egui::FullOutput {
                    shapes,
                    textures_delta,
                    ..
                } = context.end_frame();
                let paint_jobs = context.tessellate(shapes);
                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    textures_delta,
                    context: context.clone(),
                };

                // camera stuff
                camera.input(&input_manager);
                input_manager.reset_frame();
                let now = std::time::Instant::now();
                let elapsed = now.duration_since(time);
                time = now;
                let view = camera.drive(elapsed.as_secs_f32());
                // println!("{:?}", view);

                renderer.set_camera_data(rend3::types::Camera {
                    projection: rend3::types::CameraProjection::Perspective {
                        vfov: 60.0,
                        near: 0.1,
                    },
                    view,
                });

                // rendering stuff

                // Get a frame
                let frame = surface.get_current_texture().unwrap();

                // Swap the instruction buffers so that our frame's changes can be processed.
                renderer.swap_instruction_buffers();
                // Evaluate our frame's world-change instructions
                let mut eval_output = renderer.evaluate_instructions();

                // Build a rendergraph
                let mut graph = rend3::graph::RenderGraph::new();

                // Import the surface texture into the render graph.
                let frame_handle = graph.add_imported_render_target(
                    &frame,
                    0..1,
                    rend3::graph::ViewportRect::from_size(resolution),
                );

                base_rendergraph.add_to_graph(
                    &mut graph,
                    &eval_output,
                    &pbr_routine,
                    None,
                    &tonemapping_routine,
                    frame_handle,
                    resolution,
                    rend3::types::SampleCount::One,
                    Vec4::ZERO,
                    Vec4::new(0.047058824, 0.12156863, 0.23921569, 1.0),
                );

                egui_routine.add_to_graph(&mut graph, input, frame_handle);

                // Dispatch a render using the built up rendergraph!
                graph.execute(&renderer, &mut eval_output);

                // Present the frame
                frame.present();
            }

            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                input_manager.key_event(input);
                if input_manager.escape_requested {
                    *control = winit::event_loop::ControlFlow::Exit;
                }
            }

            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::MouseWheel { delta, .. },
                ..
            } => input_manager.zoom_event(delta),
            // Other events we don't care about
            _ => {}
        }
    });
}
