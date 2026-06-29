use bevy::{
    input_focus::tab_navigation::{TabIndex, TabNavigationPlugin},
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{
        Slider, SliderDragState, SliderRange, SliderThumb, SliderValue, TrackClick, observe,
        slider_self_update,
    },
};

pub const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
pub const MAIN_COLOR: Color = Color::srgb(0.1, 0.3, 0.1);

/// Add the slider plugin to enable the slider functionality.
pub struct VerticalSliderPlugin;

impl Plugin for VerticalSliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TabNavigationPlugin);
        app.add_systems(Update, (update_slider_visuals, update_value_labels));
    }
}

#[derive(Component)]
struct ValueTextId(Entity);

#[derive(Component)]
pub struct ValueLabel(pub String);

#[derive(Component)]
pub struct VerticalSlider;

#[derive(Component)]
pub struct VerticalSliderNode;

#[derive(Component)]
pub struct SliderExtents {
    min: f32,
    max: f32,
}

#[derive(Component)]
pub struct SliderScaledValue(pub f32);

pub fn spawn_vertical_slider_ui(
    label: &str,
    init_scaled: f32,
    min: f32,
    max: f32,
    commands: &mut ChildSpawnerCommands,
    assets: &AssetServer,
) {
    assert!(min <= max);
    assert!(init_scaled >= min);
    assert!(init_scaled <= max);
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(10),
                ..default()
            },
            VerticalSliderNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf").into(),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(MAIN_COLOR),
            ));

            let text_id = parent
                .spawn((
                    Text::new("50"),
                    TextFont {
                        font: assets.load("fonts/FiraSans-Bold.ttf").into(),
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(MAIN_COLOR),
                ))
                .id();

            let init_posn = (init_scaled - min) / (max - min) * 100.0;
            parent.spawn((
                vertical_slider(init_posn),
                ValueTextId(text_id),
                ValueLabel(label.to_string()),
                SliderExtents { min, max },
                SliderScaledValue(init_scaled),
                observe(slider_self_update),
            ));
        });
}

fn vertical_slider(init: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            column_gap: px(4),
            width: px(12),
            height: px(200),
            ..default()
        },
        VerticalSlider,
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
            orientation: bevy::ui_widgets::SliderOrientation::Vertical,
        },
        SliderValue(init),
        SliderRange::new(0.0, 100.0),
        TabIndex(0),
        Children::spawn((
            Spawn((
                Node {
                    width: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(MAIN_COLOR),
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    top: px(12),
                    bottom: px(0),
                    left: px(0),
                    right: px(0),
                    ..default()
                },
                children![(
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        bottom: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

fn update_slider_visuals(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &SliderDragState,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<Hovered>,
                Changed<SliderDragState>,
            )>,
            With<VerticalSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<SliderThumb>), Without<VerticalSlider>>,
) {
    for (slider_ent, value, range, hovered, drag_state) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                let position = range.thumb_position(value.0) * 100.0;
                thumb_node.bottom = percent(position);

                let is_active = hovered.0 | drag_state.dragging;
                thumb_bg.0 = if is_active {
                    SLIDER_THUMB.lighter(0.3)
                } else {
                    SLIDER_THUMB
                };
            }
        }
    }
}

fn update_value_labels(
    mut sliders: Query<
        (
            &SliderValue,
            &ValueTextId,
            &SliderExtents,
            &mut SliderScaledValue,
        ),
        Changed<SliderValue>,
    >,
    mut texts: Query<&mut Text>,
) {
    for (value, text_entity, extents, mut slider_scaled_value) in sliders.iter_mut() {
        let scaled_value = extents.min + (extents.max - extents.min) * (value.0 / 100.0);
        slider_scaled_value.0 = scaled_value;
        if let Ok(mut text) = texts.get_mut(text_entity.0) {
            **text = format!("{:.1}", scaled_value);
        }
    }
}
