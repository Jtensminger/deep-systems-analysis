use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use super::{
        SOI_FILL_COLOR,
        SOI_RADIUS,
        SOI_STROKE_COLOR,
        SOI_STROKE_SIZE
};


#[derive(Component)]
pub struct Environment;


#[derive(Component)]
pub struct SOI;

#[derive(Component)]
pub struct OutflowRequirements {
        pub product_material_outflow: u32,
        pub product_energy_outflow: u32,
        pub product_message_outflow: u32,
        pub waste_material_outflow: u32,
        pub waste_energy_outflow: u32
}

#[derive(Component)]
pub struct System;

#[derive(Component)]
pub struct Boundary;

#[derive(Component)]
pub struct SOIRenderData;


pub mod construct_spawned {
        use bevy::prelude::*;
        
        #[derive(Event)]
        pub struct SOI { pub entity: Entity }

        #[derive(Event)]
        pub struct Boundary { pub entity: Entity }

        #[derive(Event)]
        pub struct Environment { pub entity: Entity }
}


pub fn spawn_soi(
        mut commands: Commands,
        mut events: EventWriter<construct_spawned::SOI>,
) {
        let soi = commands.spawn_empty().id();
        commands.entity(soi)
                .insert(SOI)
                .insert(System)
                .insert(OutflowRequirements {
                        product_material_outflow: 1,
                        product_energy_outflow: 1,
                        product_message_outflow: 1,
                        waste_material_outflow: 0,
                        waste_energy_outflow: 0
                });
        
        events.send(construct_spawned::SOI { entity: soi });
}


pub fn handle_soi_spawn(
        mut commands: Commands,
        mut soi_events: EventReader<construct_spawned::SOI>,
) {
        for event in soi_events.read() {
                let soi = event.entity;

                let boundary = commands.spawn_empty().id();
                commands.entity(boundary)
                        .insert(Boundary);

                let environment = commands.spawn_empty().id();
                commands.entity(environment)
                        .insert(Environment);
        
                commands.entity(soi).push_children(&[boundary, environment]);
        }
}


pub fn draw_soi(
        mut commands: Commands,
        query: Query<Entity, With<SOI>>,
) {
        let soi = query.single();
        
        let shape = shapes::Circle {
                radius: SOI_RADIUS,
                center: Vec2::new(0.0, 0.0),
        };
        
        let shape_bundle = ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
        };

        commands.entity(soi)
                .insert(shape_bundle)
                .insert(Fill::color(SOI_FILL_COLOR))
                .insert(Stroke::new(SOI_STROKE_COLOR, SOI_STROKE_SIZE));
}