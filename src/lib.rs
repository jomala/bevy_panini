mod pipeline;
mod settings;

pub mod prelude {
    pub use crate::PaniniPlugin;
    pub use crate::settings::PaniniProjection;
}

use bevy::{
    core_pipeline::Core3dSystems,
    core_pipeline::schedule::Core3d,
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
    },
};

use crate::pipeline::{init_post_process_pipeline, post_process_system};
use crate::settings::{PaniniShaderSettings, copy_out_shader_settings};

/// This example uses a shader source file from the assets subdirectory
pub const SHADER_ASSET_PATH: &str = "shaders/bevy_panini.wgsl";

/// The plugin to include in your app to enable the Panini projection effect.
pub struct PaniniPlugin;

impl Plugin for PaniniPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // The settings will be a component that lives in the main world but will
            // be extracted to the render world every frame.
            // This makes it possible to control the effect from the main world.
            // This plugin will take care of extracting it automatically.
            // It's important to derive [`ExtractComponent`] on [`PaniniShaderSettings`]
            // for this plugin to work correctly.
            ExtractComponentPlugin::<PaniniShaderSettings>::default(),
            // The settings will also be the data used in the shader.
            // This plugin will prepare the component for the GPU by creating a uniform buffer
            // and writing the data to that buffer every frame.
            UniformComponentPlugin::<PaniniShaderSettings>::default(),
        ));

        // Update the setting whenever the camera projection changes.
        // Copy the shader settings out of the projection before extracting them.
        app.add_systems(PostUpdate, copy_out_shader_settings);

        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // RenderStartup runs once on startup after all plugins are built
        // It is useful to initialize data that will only live in the RenderApp
        render_app.add_systems(RenderStartup, init_post_process_pipeline);

        render_app.add_systems(
            Core3d,
            post_process_system.in_set(Core3dSystems::PostProcess),
        );
    }
}
