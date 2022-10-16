use bevy::{
    app::ScheduleRunnerSettings,
    asset::AssetPlugin,
    core::CorePlugin,
    core_pipeline::CorePipelinePlugin,
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin, LogSettings},
    prelude::*,
    render::RenderPlugin,
    scene::ScenePlugin,
    sprite::SpritePlugin,
    time::TimePlugin,
    window::{WindowId, WindowPlugin},
};
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvas;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}

impl HelloPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            ..default()
        });
    }
}

#[derive(Resource)]
struct CanvasSettings {
    offscreen_canvas: OffscreenCanvas,
}

unsafe impl Send for CanvasSettings {}
unsafe impl Sync for CanvasSettings {}

struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        bevy::log::debug!("CanvasPlugin::build");
        let settings = app.world.resource_mut::<CanvasSettings>();
        let canvas = settings.offscreen_canvas.clone();

        let mut windows = app.world.resource_mut::<bevy::window::Windows>();
        windows.add(Window::new_offscreen_canvas(
            WindowId::primary(),
            &WindowDescriptor::default(),
            canvas,
            1.0,
        ));
    }
}

#[wasm_bindgen]
pub fn start(canvas: OffscreenCanvas) {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .insert_resource(LogSettings {
            filter: "debug,wgpu=error".to_string(),
            level: Level::DEBUG,
        })
        .insert_resource(CanvasSettings {
            offscreen_canvas: canvas,
        })
        .add_plugin(LogPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(TimePlugin)
        .add_plugin(TransformPlugin)
        .add_plugin(HierarchyPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(WindowPlugin)
        .add_plugin(CanvasPlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CorePipelinePlugin)
        .add_plugin(SpritePlugin)
        .add_plugin(HelloPlugin)
        .run();
}

fn main() {}
