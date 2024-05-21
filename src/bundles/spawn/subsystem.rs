use crate::bundles::{spawn_interface, SystemBundle};
use crate::components::*;
use crate::constants::*;
use crate::events::SubsystemDrag;
use crate::plugins::label::add_name_label;
use crate::plugins::mouse_interaction::DragPosition;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator,
};
use crate::utils::{
    compute_end_and_direction_from_subsystem, transform_from_point2d_and_direction,
};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_eventlistener::event_listener::On;

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    is_child_of_interface: bool,
    interface_entity: Entity,
    flow_interface_query: &Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    system_query: &Query<(&Transform, &crate::components::System)>,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: &Res<FocusedSystem>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    name: &str,
    description: &str,
) -> Entity {
    let mut interface_flow_entity = Entity::PLACEHOLDER;
    let mut angle = 0.0;
    let mut is_import_subsystem = false;
    let mut is_export_subsystem = false;
    let mut interface_subsystem = InterfaceSubsystem::new(interface_entity);

    for (entity, flow, inflow_connection, outflow_connection) in flow_interface_query {
        if let Some(connection) = inflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                angle = std::f32::consts::PI;
                is_import_subsystem = true;
                interface_subsystem.total_inflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.is_useful;
            }
        }
        if let Some(connection) = outflow_connection {
            if connection.target == interface_entity {
                interface_flow_entity = entity;
                is_export_subsystem = true;
                interface_subsystem.total_outflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.is_useful;
            }
        }
    }

    let parent_system = ***focused_system;

    let z = if is_child_of_interface {
        SUBSYSTEM_Z - INTERFACE_Z
    } else {
        SUBSYSTEM_Z
    };

    let (subsystem_entity, _, _) = spawn_subsystem_common(
        commands,
        system_query,
        nesting_level_query,
        meshes,
        zoom,
        name,
        description,
        angle,
        parent_system,
        z,
        SubsystemPosition::XFromRadius,
    );

    let mut subsystem_commands = commands.entity(subsystem_entity);

    subsystem_commands.insert((
        interface_subsystem,
        SubsystemParentFlowConnection {
            target: interface_flow_entity,
        },
    ));

    if is_import_subsystem {
        subsystem_commands.insert(ImportSubsystem);
    }
    if is_export_subsystem {
        subsystem_commands.insert(ExportSubsystem);
    }

    let subsystem_entity = subsystem_commands.id();

    let mut interface_commands = commands.entity(interface_entity);
    interface_commands.insert(InterfaceSubsystemConnection {
        target: subsystem_entity,
    });

    if is_child_of_interface {
        interface_commands.add_child(subsystem_entity);
    } else {
        commands.entity(parent_system).add_child(subsystem_entity);
    }

    subsystem_entity
}

enum SubsystemPosition {
    XFromRadius,
    Position(Vec2),
}

fn spawn_subsystem_common(
    commands: &mut Commands,
    system_query: &Query<(&Transform, &crate::components::System)>,
    nesting_level_query: &Query<&NestingLevel>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    name: &str,
    description: &str,
    angle: f32,
    parent_system: Entity,
    z: f32,
    position: SubsystemPosition,
) -> (Entity, f32, u16) {
    let radius = system_query
        .get(parent_system)
        .expect("focused system not found")
        .1
        .radius
        * SUBSYSTEM_RADIUS_FRACTION;

    let nesting_level = NestingLevel::current(parent_system, nesting_level_query) + 1;

    let position = match position {
        SubsystemPosition::XFromRadius => vec2(-radius * zoom, 0.0),
        SubsystemPosition::Position(position) => position,
    };

    (
        commands
            .spawn((
                Subsystem { parent_system },
                NestingLevel::new(nesting_level),
                SystemBundle::new(
                    position,
                    z,
                    radius,
                    angle,
                    false,
                    false,
                    Default::default(),
                    meshes,
                    zoom,
                    nesting_level,
                    name,
                    description,
                ),
                Pinnable { has_pins: false },
                On::<DragPosition>::send_event::<SubsystemDrag>(),
            ))
            .id(),
        radius,
        nesting_level,
    )
}

pub fn spawn_subsystem(
    commands: &mut Commands,
    parent_system: Entity,
    system_query: &Query<(&Transform, &crate::components::System)>,
    nesting_level_query: &Query<&NestingLevel>,
    flow_query: &Query<(&FlowCurve, &Flow)>,
    inflows: &[Entity],
    outflows: &[Entity],
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    zoom: f32,
    name: &str,
    description: &str,
    position: Vec2,
) {
    let (subsystem_entity, radius, nesting_level) = spawn_subsystem_common(
        commands,
        system_query,
        nesting_level_query,
        meshes,
        zoom,
        name,
        description,
        0.0,
        parent_system,
        SUBSYSTEM_Z,
        SubsystemPosition::Position(position),
    );

    let zoomed_radius = radius * zoom;

    commands.entity(parent_system).add_child(subsystem_entity);

    for inflow in inflows {
        let (flow_curve, flow) = flow_query.get(*inflow).expect("Inflow not found");

        let (_, dir) = compute_end_and_direction_from_subsystem(
            position,
            zoomed_radius,
            flow_curve.start,
            flow_curve.start_direction,
        );
        let transform = transform_from_point2d_and_direction(dir * zoomed_radius, dir);

        spawn_interface(
            commands,
            InterfaceType::Import,
            flow.substance_type,
            *inflow,
            &transform,
            nesting_level,
            subsystem_entity,
            fixed_system_element_geometries,
            zoom,
            true,
            meshes,
            tess,
            "Interface",
            "",
        );

        commands.entity(*inflow).insert(FlowEndConnection {
            target: subsystem_entity,
            target_type: EndTargetType::System,
        });
    }

    for outflow in outflows {
        let (flow_curve, flow) = flow_query.get(*outflow).expect("Outflow not found");

        let (_, dir) = compute_end_and_direction_from_subsystem(
            position,
            zoomed_radius,
            flow_curve.end,
            flow_curve.end_direction,
        );
        let transform = transform_from_point2d_and_direction(dir * zoomed_radius, dir);

        spawn_interface(
            commands,
            InterfaceType::Export,
            flow.substance_type,
            *outflow,
            &transform,
            nesting_level,
            subsystem_entity,
            fixed_system_element_geometries,
            zoom,
            true,
            meshes,
            tess,
            "Interface",
            "",
        );

        commands.entity(*outflow).insert(FlowStartConnection {
            target: subsystem_entity,
            target_type: StartTargetType::System,
        });
    }
}

pub fn auto_spawn_interface_subsystem_label(
    mut commands: Commands,
    interface_subsystem_query: Query<Entity, Added<InterfaceSubsystem>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for interface_subsystem in interface_subsystem_query.iter() {
        add_name_label(
            &mut commands,
            interface_subsystem,
            vec2(100.0, 100.0),
            vec3(0.0, 0.0, 0.0),
            &name_query,
            &asset_server,
        );
    }
}

pub fn auto_spawn_subsystem_label(
    mut commands: Commands,
    subsystem_query: Query<Entity, Added<Subsystem>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for subsystem in subsystem_query.iter() {
        add_name_label(
            &mut commands,
            subsystem,
            vec2(100.0, 100.0),
            vec3(0.0, 0.0, 0.0),
            &name_query,
            &asset_server,
        );
    }
}
