use crate::components::{
    EndTargetType, FlowEndConnection, FlowEndInterfaceConnection, FlowStartConnection,
    FlowStartInterfaceConnection, InitialPosition, InterfaceSubsystem, StartTargetType,
};
use crate::constants::{FLOW_END_LENGTH, INTERFACE_WIDTH_HALF};
use bevy::prelude::*;

pub fn ui_transform_from_button(
    button_transform: &Transform,
    z: f32,
    move_right: f32,
    zoom: f32,
) -> (Transform, InitialPosition) {
    let position = button_transform.translation.truncate() / zoom;
    let right = button_transform.right().truncate();

    let position = position + right * move_right;
    (
        Transform::from_translation((position * zoom).extend(z))
            .with_rotation(button_transform.rotation),
        InitialPosition::new(position),
    )
}

pub fn compute_end_and_direction_from_system_child(
    system_child: Entity,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&Parent>,
    flow_parent: Option<Entity>,
    zoom: f32,
    scale: f32,
) -> (Vec2, Vec2) {
    let combined_transform = combined_transform_of_entity_until_ancestor(
        system_child,
        flow_parent,
        transform_query,
        parent_query,
    );

    let right = combined_transform.right().truncate();

    (
        combined_transform.translation.truncate() + right * INTERFACE_WIDTH_HALF * scale,
        right * FLOW_END_LENGTH * zoom,
    )
}

pub fn combined_transform_of_entity_until_ancestor(
    entity: Entity,
    ancestor: Option<Entity>,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&Parent>,
) -> Transform {
    let mut combined_transform = *transform_query
        .get(entity)
        .expect("Entity should have a Transform");
    let mut parent_entity = parent_query
        .get(entity)
        .expect("Entity should have a Parent")
        .get();

    loop {
        let parent_transform = transform_query
            .get(parent_entity)
            .expect("Parent should have a Transform");

        combined_transform = parent_transform.mul_transform(combined_transform);

        if let Ok(parent) = parent_query.get(parent_entity) {
            parent_entity = parent.get();
        } else {
            break;
        }

        if Some(parent_entity) == ancestor {
            break;
        }
    }

    combined_transform
}

macro_rules! all_flow_connected_systems {
    (
        $fn_name:ident,
        $conn_ty:ty,
        $interface_conn_ty:ty,
        $target_ty:tt
    ) => {
        pub fn $fn_name(
            flow: (Option<&$conn_ty>, Option<&$interface_conn_ty>),
            interface_subsystem_query: &Query<(Entity, &InterfaceSubsystem)>,
        ) -> Vec<Entity> {
            let mut connected_systems = vec![];

            let (connection, interface_connection) = flow;

            if let Some(connection) = connection {
                if matches!(connection.target_type, $target_ty::System) {
                    connected_systems.push(connection.target);
                }
            }

            if let Some(interface_connection) = interface_connection {
                let interface_entity = interface_connection.target;
                for (subsystem_entity, interface_subsystem) in interface_subsystem_query {
                    if interface_subsystem.interface_entity == interface_entity {
                        connected_systems.push(subsystem_entity);
                    }
                }
            }

            connected_systems
        }
    };
}

all_flow_connected_systems!(
    all_flow_end_connected_systems,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    EndTargetType
);
all_flow_connected_systems!(
    all_flow_start_connected_systems,
    FlowStartConnection,
    FlowStartInterfaceConnection,
    StartTargetType
);
