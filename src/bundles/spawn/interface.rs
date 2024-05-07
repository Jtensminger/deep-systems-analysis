use crate::components::*;
use crate::constants::{INTERFACE_LINE_WIDTH, INTERFACE_SELECTED_LINE_WIDTH, INTERFACE_Z};
use crate::events::InterfaceDrag;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::plugins::mouse_interaction::DragPosition;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, StrokeTessellator,
};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_interface(
    commands: &mut Commands,
    interface_type: InterfaceType,
    substance_type: SubstanceType,
    flow_entity: Entity,
    transform: &Transform,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: Entity,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: f32,
    is_selected: bool,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    name: &str,
    description: &str,
) -> Entity {
    let (mut transform, initial_position) =
        ui_transform_from_button(transform, INTERFACE_Z, 0.0, zoom);

    // Normalize the rotation
    transform.rotation = Quat::from_rotation_z(transform.translation.truncate().to_angle());

    let nesting_level = NestingLevel::current(focused_system, nesting_level_query);
    let scale = NestingLevel::compute_scale(nesting_level, zoom);

    let interface_entity = commands
        .spawn((
            Interface::default(),
            SpatialBundle {
                transform,
                ..default()
            },
            Fill::color(substance_type.interface_color()),
            PickableBundle::default(),
            PickSelection { is_selected },
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, INTERFACE_LINE_WIDTH * scale),
                selected: Stroke::new(Color::BLACK, INTERFACE_SELECTED_LINE_WIDTH),
            },
            SystemElement::Interface,
            Name::new(name.to_string()),
            ElementDescription::new(description),
            initial_position,
            fixed_system_element_geometries
                .get_or_create(nesting_level, zoom, meshes, tess)
                .interface
                .clone(),
            NestingLevel::new(nesting_level),
            On::<DragPosition>::send_event::<InterfaceDrag>(),
        ))
        .id();

    commands
        .entity(focused_system)
        .add_child(interface_entity);

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(FlowEndInterfaceConnection {
                target: interface_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(FlowStartInterfaceConnection {
                target: interface_entity,
            });
        }
    }

    interface_entity
}
