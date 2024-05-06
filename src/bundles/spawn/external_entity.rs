use crate::components::*;
use crate::constants::{
    BUTTON_WIDTH_HALF, EXTERNAL_ENTITY_LINE_WIDTH, EXTERNAL_ENTITY_SELECTED_LINE_WIDTH,
    EXTERNAL_ENTITY_WIDTH_HALF, EXTERNAL_ENTITY_Z,
};
use crate::events::ExternalEntityDrag;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::plugins::mouse_interaction::DragPosition;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator,
};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_external_entity(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: &Res<FocusedSystem>,
    interface_type: InterfaceType,
    substance_type: SubstanceType,
    flow_entity: Entity,
    transform: &Transform,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: f32,
    is_selected: bool,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
) -> Entity {
    let (transform, initial_position) = ui_transform_from_button(
        transform,
        EXTERNAL_ENTITY_Z,
        EXTERNAL_ENTITY_WIDTH_HALF - BUTTON_WIDTH_HALF,
        zoom,
    );

    let color = substance_type.flow_color();

    let nesting_level = NestingLevel::current(***focused_system, nesting_level_query);
    let scale = NestingLevel::compute_scale(nesting_level, zoom);

    let external_entity = commands
        .spawn((
            ExternalEntity,
            SpatialBundle {
                transform,
                ..default()
            },
            HighlightBundles {
                idle: Stroke::new(color, EXTERNAL_ENTITY_LINE_WIDTH * scale),
                selected: Stroke {
                    color,
                    options: StrokeOptions::default()
                        .with_line_width(EXTERNAL_ENTITY_SELECTED_LINE_WIDTH * scale)
                        .with_line_cap(LineCap::Round),
                },
            },
            PickableBundle::default(),
            PickSelection { is_selected },
            SystemElement::ExternalEntity,
            Name::new("External Entity"),
            ElementDescription::default(),
            initial_position,
            fixed_system_element_geometries
                .get_or_create(nesting_level, zoom, meshes, tess)
                .external_entity,
            NestingLevel::new(nesting_level),
            On::<DragPosition>::send_event::<ExternalEntityDrag>(),
        ))
        .id();

    if let Ok(subsystem) = subsystem_query.get(***focused_system) {
        commands
            .entity(subsystem.parent_system)
            .add_child(external_entity);
    }

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(InflowSourceConnection {
                target: external_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(OutflowSinkConnection {
                target: external_entity,
            });
        }
    }

    external_entity
}
