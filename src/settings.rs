/// The settings for the Panini projection effect.
use bevy::{
    prelude::*, 
    render::{
        extract_component::ExtractComponent,
        render_resource::*,
    }
};

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PaniniSettings {
    pub panini: f32,
    pub fov_x_radians: f32,
    pub aspect_ratio: f32,
    // WebGL2 structs must be 16 byte aligned.
    // cfg(feature = "webgl2")]
    _webgl2_padding: f32,
}

impl PaniniSettings {
    pub fn new(panini: f32) -> Self {
        Self {
            panini,
            fov_x_radians: 90.0f32.to_radians(),
            aspect_ratio: 16.0 / 9.0,
            _webgl2_padding: 0.0,
        }
    }

    pub fn update(&mut self, panini: f32) {
        self.panini = panini;
    }
}

pub fn update_settings(mut query: Query<(&mut PaniniSettings, &Projection)>) {
    for (mut settings, projection) in &mut query {
        // Update the FOV and aspect ration based on the camera's projection.
        if let Projection::Perspective(perspective_projection) = projection {
            settings.fov_x_radians = perspective_projection.fov * perspective_projection.aspect_ratio;
            settings.aspect_ratio = perspective_projection.aspect_ratio;
        }
    }
}
