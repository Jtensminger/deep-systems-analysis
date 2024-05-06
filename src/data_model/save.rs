use crate::components::*;
use crate::data_model::*;
use bevy::core::Name;
use bevy::prelude::*;

pub fn save_world(
    soi_info_query: Query<
        (
            Entity,
            &Name,
            &ElementDescription,
            &crate::components::System,
            &SystemEnvironment,
            &Transform,
        ),
        Without<Subsystem>,
    >,
    inflow_query: Query<(
        &Name,
        &ElementDescription,
        &Inflow,
        &InflowSourceConnection,
        &InflowInterfaceConnection,
    )>,
    outflow_query: Query<(
        &Name,
        &ElementDescription,
        &Outflow,
        &OutflowSinkConnection,
        &OutflowInterfaceConnection,
    )>,
    interface_query: Query<(
        &Name,
        &ElementDescription,
        &crate::components::Interface,
        &Transform,
    )>,
    external_entity_query: Query<(&Name, &ElementDescription, &Transform)>,
    // subsystem_query: Query<
    //     (Entity, &Name, &crate::components::ElementDescription, &crate::components::SystemElement),
    //     (With<crate::components::Subsystem>, Without<crate::components::SystemOfInterest>)
    // >
) {
    let (system_entity, name, description, system, environment, transform) = soi_info_query
        .get_single()
        .expect("System of interest should exist");

    let mut interfaces = vec![];
    let mut external_interactions = vec![];
    let mut sources = vec![];
    let mut sinks = vec![];

    for (name, description, flow, source_connection, interface_connection) in &inflow_query {
        process_external_inflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sources,
            name,
            description,
            flow,
            source_connection,
            interface_connection,
        );
    }

    for (name, description, flow, sink_connection, interface_connection) in &outflow_query {
        process_external_outflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sinks,
            name,
            description,
            flow,
            sink_connection,
            interface_connection,
        );
    }

    let boundary = Boundary {
        info: Info {
            id: Id {
                ty: IdType::Boundary,
                indices: vec![0],
            },
            level: 0,
            name: system.boundary.name.clone(),
            description: system.boundary.description.clone(),
        },
        porosity: system.boundary.porosity,
        perceptive_fuzziness: system.boundary.perceptive_fuzziness,
        interfaces,
    };

    let environment = Environment {
        info: Info {
            id: Id {
                ty: IdType::Environment,
                indices: vec![-1],
            },
            name: environment.name.clone(),
            description: environment.description.clone(),
            level: -1,
        },
        sources,
        sinks,
    };

    let system_of_interest = crate::data_model::System {
        info: Info {
            id: Id {
                ty: IdType::System,
                indices: vec![0],
            },
            level: 0,
            name: name.to_string(),
            description: description.text.clone(),
        },
        parent: None,
        complexity: Complexity::Complex {
            adaptable: system.adaptable,
            evolveable: system.evolveable,
        },
        environment,
        boundary,
        internal_interactions: vec![], // TODO
        external_interactions,
        components: vec![],
        transform: Some(transform.into()),
    };

    let model = WorldModel { system_of_interest };
    save_to_json(&model, "world_model.json");
}

macro_rules! process_external_flow {
    (
        $fn_name:ident,
        $flow_ty:ty,
        $interface_connection_ty:ty,
        $external_entity_connection_ty:ty,
        $interaction_ty:tt,
        $interface_ty:expr,
        $external_entity_ty:expr,
        $id_ty:expr,
        $external_entity_field:tt
    ) => {
        fn $fn_name(
            interface_query: &Query<(
                &Name,
                &ElementDescription,
                &crate::components::Interface,
                &Transform,
            )>,
            external_entity_query: &Query<(&Name, &ElementDescription, &Transform)>,
            system_entity: Entity,
            interfaces: &mut Vec<crate::data_model::Interface>,
            interactions: &mut Vec<crate::data_model::Interaction>,
            external_entities: &mut Vec<crate::data_model::ExternalEntity>,
            name: &Name,
            description: &ElementDescription,
            flow: &$flow_ty,
            source_connection: &$external_entity_connection_ty,
            interface_connection: &$interface_connection_ty,
        ) {
            if flow.system == system_entity {
                let interaction_id = Id {
                    ty: IdType::Flow,
                    indices: vec![-1, interactions.len() as i64],
                };
                let external_entity_id = Id {
                    ty: $id_ty,
                    indices: vec![-1, external_entities.len() as i64],
                };

                interactions.push(crate::data_model::Interaction {
                    info: Info {
                        id: interaction_id.clone(),
                        name: name.to_string(),
                        description: description.text.clone(),
                        level: -1,
                    },
                    substance: Substance {
                        sub_type: None, // TODO
                        ty: flow.substance_type,
                    },
                    ty: InteractionType::$interaction_ty {
                        usability: flow.usability,
                    },
                    external_entity: external_entity_id.clone(),
                });

                let (interface_name, interface_description, interface, interface_transform) =
                    interface_query
                        .get(interface_connection.target)
                        .expect("Interface should exist");

                let mut interface = crate::data_model::Interface {
                    info: Info {
                        id: Id {
                            ty: IdType::Interface,
                            indices: vec![0, interfaces.len() as i64],
                        },
                        name: interface_name.to_string(),
                        description: interface_description.text.clone(),
                        level: 1,
                    },
                    protocol: interface.protocol.clone(),
                    ty: $interface_ty,     // TODO: hybrid
                    receives_from: vec![], // TODO : multiple
                    exports_to: vec![],    // TODO
                    angle: Some(interface_transform.right().truncate().to_angle()),
                };

                interface
                    .$external_entity_field
                    .push(external_entity_id.clone());

                interfaces.push(interface);

                let (external_entity_name, external_entity_description, external_entity_transform) =
                    external_entity_query
                        .get(source_connection.target)
                        .expect("External entity should exist");

                external_entities.push(crate::data_model::ExternalEntity {
                    info: Info {
                        id: external_entity_id,
                        name: external_entity_name.to_string(),
                        description: external_entity_description.text.clone(),
                        level: -1,
                    },
                    ty: $external_entity_ty,
                    interactions: vec![interaction_id], // TODO : multiple
                    transform: Some(external_entity_transform.into()),
                });
            }
        }
    };
}

process_external_flow!(
    process_external_inflow,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    Inflow,
    crate::data_model::InterfaceType::Import,
    ExternalEntityType::Source,
    IdType::Source,
    receives_from
);

process_external_flow!(
    process_external_outflow,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    Outflow,
    crate::data_model::InterfaceType::Export,
    ExternalEntityType::Sink,
    IdType::Sink,
    exports_to
);

fn save_to_json(world_model: &WorldModel, file_name: &str) {
    let json = serde_json::to_string(world_model).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}