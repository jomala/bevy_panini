/// The custom Projection component for the Panini projection effect. 

use bevy::{
    camera::CameraProjection,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::*,
    }
};

/// This component can be initialized and update by the user as a `Projection::Custom` projection on the camera.
#[derive(Debug, Clone)]
pub struct PaniniProjection {
    settings: PaniniShaderSettings,
    enabled: bool,
    perspective: PerspectiveProjection,
}

impl CameraProjection for PaniniProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        self.perspective.get_clip_from_view()
    }

    fn get_clip_from_view_for_sub(&self, sub_view: &bevy::camera::SubCameraView) -> Mat4 {
        let perspective_sub_view = sub_view; // TODO: Wrong?
        self.perspective.get_clip_from_view_for_sub(perspective_sub_view)
    }

    /// This update method is called by the camera system whenever the viewport is resized.
    fn update(&mut self, width: f32, height: f32) {
        self.perspective.update(width, height);
        self.settings.update_settings(self.perspective.fov, self.perspective.aspect_ratio);
    }

    fn far(&self) -> f32 {
        self.perspective.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        self.perspective.get_frustum_corners(z_near, z_far)
    }
}

impl PaniniProjection {
    /// Create a new Panini projection with the given settings.
    /// This keys off the vertical field of view, the aspect ratio, the Panini depth parameter. The horizontal field of view is calculated based on these parameters.
    /// The default is the default projection with no Panini effect (panini depth of 0).
    // TODO: Cannot currently change the clipping planes.
    pub fn new() -> Self {
        let perspective = PerspectiveProjection::default();
        let settings = PaniniShaderSettings::new(0.0, perspective.fov, perspective.aspect_ratio);
        Self { 
            settings,
            enabled: true,
            perspective,
        }
    }

    /// Set the initial vertical field of view. 
    pub fn with_fov_y(mut self, fov_y_radians: f32) -> Self {
        self.update_fov_y(fov_y_radians);
        self
    }

    /// Update the vertical field of view. The horizontal field of view will be updated automatically based on the aspect ratio and the Panini depth parameter.
    pub fn update_fov_y(&mut self, fov_y_radians: f32) {
        self.perspective.fov = fov_y_radians;
        self.settings.update_settings(fov_y_radians, self.perspective.aspect_ratio);
    }

    /// Set the initial Panini depth parameter. This will effect the horizontal field of view and the amount of distortion and blur.
    pub fn with_panini_depth(mut self, panini: f32) -> Self{
        self.update_panini_depth(panini);
        self
    }
    /// Update the Panini depth parameter. This will effect the horizontal field of view and the amount of distortion and blur.
    pub fn update_panini_depth(&mut self, panini: f32) {
        self.settings.update_panini_depth(panini);
    }
    
    /// Set the initial compression parameter. This will effect the straightness of lines and the amount of distortion and blur.
    pub fn with_compression(mut self, comp: f32) -> Self{
        self.update_compression(comp);
        self
    }
    /// Update the compression parameter. This will effect the straightness of lines and the amount of distortion and blur.
    pub fn update_compression(&mut self, comp: f32) {
        self.settings.update_compression(comp);
    }
    
    /// Set the initial enabled parameter. This is a master switch for the post-processing. If disabled the perspective projection from this class remains in use.
    pub fn with_enabled(mut self, enabled: bool) -> Self{
        self.update_enabled(enabled);
        self
    }
    /// Update the initial enabled parameter. This is a master switch for the post-processing. If disabled the perspective projection from this class remains in use.
    pub fn update_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Check the initial enabled parameter. This is a master switch for the post-processing. If disabled the perspective projection from this class remains in use.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Component, Default, Debug, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PaniniShaderSettings {
    pub panini: f32,
    pub fov_x_radians: f32,
    pub viewport_aspect_ratio: f32,
    pub compression: f32,
    // WebGL2 structs must be 16 byte aligned.
    // cfg(feature = "webgl2")]
}

impl PaniniShaderSettings {
    pub fn new(panini: f32, fov_y_radians: f32, aspect_ratio: f32) -> Self {
        assert!(panini >= 0.0 && panini <= 1.0, "Panini settings must be between 0 and 1");
        assert!(fov_y_radians > 0.0 && fov_y_radians < std::f32::consts::PI, "FOV must be between 0 and 180 degrees");
        assert!(aspect_ratio > 0.0, "Aspect ratio must be positive");

        let fov_x_radians = Self::convert_fov_y_to_fov_x(fov_y_radians, aspect_ratio);

        Self {
            panini,
            fov_x_radians,
            viewport_aspect_ratio: aspect_ratio,
            compression: 0.0,
        }
    }

    pub fn update_settings(&mut self, fov_y_radians: f32, aspect_ratio: f32) {
        self.fov_x_radians = Self::convert_fov_y_to_fov_x(fov_y_radians, aspect_ratio);
        self.viewport_aspect_ratio = aspect_ratio;
    }

    fn convert_fov_y_to_fov_x(fov_y_radians: f32, aspect_ratio: f32) -> f32 {
        let cam_z = 1.0 / (fov_y_radians * 0.5).tan();
        (aspect_ratio / cam_z).atan() * 2.0
    }

    pub fn update_panini_depth(&mut self, panini: f32) {
        self.panini = panini;
    }

    pub fn update_compression(&mut self, comp: f32) {
        self.compression = comp;
    }
}

pub fn copy_out_shader_settings(
    mut commands: Commands,
    query: Query<(Entity, &Projection)>
) {
    for (entity, projection) in &query {
        if let Projection::Custom(custom_projection) = projection {
            if let Some(panini_projection) = custom_projection.get::<PaniniProjection>() {
                if panini_projection.enabled {
                    commands.get_entity(entity).expect("Entity should persist").insert(panini_projection.settings.clone());
                } else {
                    commands.get_entity(entity).expect("Entity should persist").remove::<PaniniShaderSettings>();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn convert_fov_y_to_fov_x_with_square_aspect_ratio_returns_same_fov() {
        let fov_y = std::f32::consts::FRAC_PI_2; // 90 degrees
        let aspect_ratio = 1.0;
        let fov_x = PaniniShaderSettings::convert_fov_y_to_fov_x(fov_y, aspect_ratio);

        assert!((fov_x - fov_y).abs() < EPSILON);
    }

    #[test]
    fn convert_fov_y_to_fov_x_with_wider_aspect_ratio_returns_larger_fov_x() {
        let fov_y = std::f32::consts::FRAC_PI_2; // 90 degrees
        let aspect_ratio = 2.0;
        let expected_fov_x = 2.0 * (aspect_ratio * (fov_y * 0.5).tan()).atan();
        let fov_x = PaniniShaderSettings::convert_fov_y_to_fov_x(fov_y, aspect_ratio);

        assert!((fov_x - expected_fov_x).abs() < EPSILON);
        assert!(fov_x > fov_y);
    }

    #[test]
    fn convert_fov_y_to_fov_x_with_narrow_fov_returns_small_value() {
        let fov_y = 0.1;
        let aspect_ratio = 1.5;
        let fov_x = PaniniShaderSettings::convert_fov_y_to_fov_x(fov_y, aspect_ratio);
        let expected_fov_x = 2.0 * (aspect_ratio * (fov_y * 0.5).tan()).atan();

        assert!((fov_x - expected_fov_x).abs() < EPSILON);
        assert!(fov_x > 0.0);
    }
    
    #[test]
    fn convert_fov_y_to_fov_x_with_180_fov_returns_180() {
        let fov_y = PI - 0.01; // Just under 180 degrees to avoid tan(90) singularity
        let aspect_ratio = 1.5;
        let fov_x = PaniniShaderSettings::convert_fov_y_to_fov_x(fov_y, aspect_ratio);

        assert!(fov_x > fov_y);
        assert!(fov_x < PI);
    }
}
