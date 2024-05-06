use crate::components::*;
use crate::constants::*;
use crate::plugins::mouse_interaction::{PickParent, PickSelection};
use crate::resources::*;
use crate::systems::create_path_from_flow_curve;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_selected_interface(
    mut commands: Commands,
    interface_query: Query<
        (Entity, &PickSelection, &NestingLevel),
        (
            With<Interface>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometriesByNestingLevel>,
) {
    for (selected_entity, selection, nesting_level) in &interface_query {
        if selection.is_selected {
            let helper_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    PickParent,
                    fixed_system_element_geometries
                        .get(&**nesting_level)
                        .expect("Geometries added in spawn_interface")
                        .interface
                        .clone(),
                    Stroke::new(Color::WHITE, INTERFACE_SELECTED_INNER_LINE_WIDTH),
                ))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn spawn_selected_flow(
    mut commands: Commands,
    curve_query: Query<
        (Entity, &FlowCurve, &PickSelection, &NestingLevel),
        (
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    zoom: Res<Zoom>,
) {
    for (selected_entity, flow_curve, selection, nesting_level) in &curve_query {
        if selection.is_selected {
            let curve_path = create_path_from_flow_curve(
                flow_curve,
                NestingLevel::compute_scale(**nesting_level, **zoom),
            );

            let helper_entity = commands
                .spawn((
                    ShapeBundle {
                        path: curve_path,
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Stroke::new(Color::WHITE, FLOW_SELECTED_INNER_LINE_WIDTH),
                ))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn spawn_selected_external_entity(
    mut commands: Commands,
    external_entity_query: Query<
        (Entity, &PickSelection, &NestingLevel),
        (
            With<ExternalEntity>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometriesByNestingLevel>,
) {
    for (selected_entity, selection, nesting_level) in &external_entity_query {
        if selection.is_selected {
            let helper_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    fixed_system_element_geometries
                        .get(&**nesting_level)
                        .expect("Geometries have to be created already by spawn_external_entity")
                        .external_entity
                        .clone(),
                    PickParent,
                    Stroke::new(Color::WHITE, EXTERNAL_ENTITY_SELECTED_INNER_LINE_WIDTH),
                ))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn update_selected_flow_curve(
    flow_curve_query: Query<
        (&FlowCurve, &SelectedHighlightHelperAdded, &NestingLevel),
        Changed<FlowCurve>,
    >,
    mut selected_query: Query<&mut Path>,
    zoom: Res<Zoom>,
) {
    for (flow_curve, helper, nesting_level) in &flow_curve_query {
        let mut path = selected_query
            .get_mut(helper.helper_entity)
            .expect("Helper entity should exist");
        let curve_path = create_path_from_flow_curve(
            flow_curve,
            NestingLevel::compute_scale(**nesting_level, **zoom),
        );

        *path = curve_path;
    }
}

pub fn despawn_selected_helper(
    mut commands: Commands,
    selected_query: Query<
        (Entity, &SelectedHighlightHelperAdded, &PickSelection),
        Changed<PickSelection>,
    >,
) {
    for (deselected_entity, helper, selection) in &selected_query {
        if !selection.is_selected {
            commands.entity(helper.helper_entity).despawn_recursive();
            commands
                .entity(deselected_entity)
                .remove::<SelectedHighlightHelperAdded>();
        }
    }
}
