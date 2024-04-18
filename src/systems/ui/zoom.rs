use crate::components::{InitialPosition, ScaleWithZoom, ZoomIndependentStrokeWidth};
use crate::resources::Zoom;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn apply_zoom(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            Option<&ScaleWithZoom>,
            Option<&InitialPosition>,
        ),
        Without<Camera>,
    >,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (entity, mut transform, scale_with_zoom, initial_position) in &mut query {
        let initial_position = if let Some(initial_position) = initial_position {
            **initial_position
        } else {
            let pos2d = transform.translation.truncate();
            commands.entity(entity).insert(InitialPosition::new(pos2d));
            pos2d
        };

        transform.translation = (initial_position * **zoom).extend(transform.translation.z);

        if let Some(scale_with_zoom) = scale_with_zoom {
            transform.scale = (**scale_with_zoom * **zoom).extend(transform.scale.z);
        }
    }
}

pub fn apply_zoom_to_stroke(
    mut query: Query<(&mut Stroke, &ZoomIndependentStrokeWidth)>,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut stroke, width) in &mut query {
        stroke.options.line_width = **width / **zoom;
    }
}