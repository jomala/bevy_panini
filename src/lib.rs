mod pipeline;
mod settings;

pub mod prelude {
    pub use crate::PaniniPlugin;
    pub use crate::settings::PaniniSettings;
}

use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        extract_component::{
            ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            RenderGraphExt, ViewNodeRunner,
        },
        RenderApp, RenderStartup,
    },
};

use crate::pipeline::{PaniniLabel, PaniniNode, init_post_process_pipeline};
use crate::settings::PaniniSettings;   

/// This example uses a shader source file from the assets subdirectory
pub const SHADER_ASSET_PATH: &str = "shaders/bevy_panini.wgsl";

/// It is generally encouraged to set up post processing effects as a plugin
pub struct PaniniPlugin;

impl Plugin for PaniniPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // The settings will be a component that lives in the main world but will
            // be extracted to the render world every frame.
            // This makes it possible to control the effect from the main world.
            // This plugin will take care of extracting it automatically.
            // It's important to derive [`ExtractComponent`] on [`PaniniingSettings`]
            // for this plugin to work correctly.
            ExtractComponentPlugin::<PaniniSettings>::default(),
            // The settings will also be the data used in the shader.
            // This plugin will prepare the component for the GPU by creating a uniform buffer
            // and writing the data to that buffer every frame.
            UniformComponentPlugin::<PaniniSettings>::default(),
        ));

        // Update the setting whenever the camera projection changes.
        app.add_systems(Update, settings::update_settings);

        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // RenderStartup runs once on startup after all plugins are built
        // It is useful to initialize data that will only live in the RenderApp
        render_app.add_systems(RenderStartup, init_post_process_pipeline);

        render_app
            // Bevy's renderer uses a render graph which is a collection of nodes in a directed acyclic graph.
            // It currently runs on each view/camera and executes each node in the specified order.
            // It will make sure that any node that needs a dependency from another node
            // only runs when that dependency is done.
            //
            // Each node can execute arbitrary work, but it generally runs at least one render pass.
            // A node only has access to the render world, so if you need data from the main world
            // you need to extract it manually or with the plugin like above.
            // Add a [`Node`] to the [`RenderGraph`]
            // The Node needs to impl FromWorld
            //
            // The [`ViewNodeRunner`] is a special [`Node`] that will automatically run the node for each view
            // matching the [`ViewQuery`]
            .add_render_graph_node::<ViewNodeRunner<PaniniNode>>(
                // Specify the label of the graph, in this case we want the graph for 3d
                Core3d,
                // It also needs the label of the node
                PaniniLabel,
            )
            .add_render_graph_edges(
                Core3d,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                (
                    Node3d::Tonemapping,
                    PaniniLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }
}
