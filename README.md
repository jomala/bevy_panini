# Bevy Panini

Custom projection and post-processing to avoid painful distortions in 3D with a wide field of view (FOV) and rectilinear perspective projection onto a flat plane, as detailed in
[this research paper](https://www.researchgate.net/publication/220795340_Pannini_A_New_Projection_for_RenderingWide_Angle_Perspective_Images).
See also [this video](https://www.youtube.com/watch?v=LE9kxUQ-l14).

![Panini reprojection diagram](panini.png)

A Panini depth of 0.0 is a normal flat perspective projection onto a plane. A positive Panini depth is a projection onto a cylinder centred on the camera, and the reprojected from
further back. This compresses the left and right edges horizontally while leaving verticals vertical (when looking at the horizon). It also reduces the vertical FOV in the centre of the screen.

Note that the effect is achieved by post-processing a flat projection so extreme FOV and high Panini depths may result in black bars where the mapping is not possible or low resolution
where the image is being distorted most. Clipping of objects may be more of an issue too.

## Usage

* Include the crate `bevy_panini`.
* Add the plugin `PaniniPlugin`.
* Give your 3D camera the custom projection `PaniniProjection` using the component `Projection::custom(PaniniProjection::new().with_panini_depth(0.5).with_fov_y(0.8))` for instance.
* Select the vertical field-of-view (FOV) and Panini depth as required. The horizontal FOV will be calculated from these and the aspect ratio of the view.
* Changing the shape of the viewport (e.g. resizing the window) does not change the vertical FOV and but does change the horizontal FOV and therefore the distortion.

See the example `skyscrapers` using `cargo run --example skyscrapers`.

* This demonstrates how to use the library.
* It also provides a mechanism for testing the visual effect of the library.
* Note that if the Panini depth is set to zero then the post-processing is disabled.

![Skyscrapers example](skyscrapers.png)

## Compatibility with Bevy

The current version of this crate is compatible with Bevy v0.19.

## Contributions

This is a work in progress. Contributions, comments and issues are very welcome, especially from those that have used Panini projections in the past!

## License

This repository is dual-licensed under either:

* [MIT License](http://opensource.org/licenses/MIT)
* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.
