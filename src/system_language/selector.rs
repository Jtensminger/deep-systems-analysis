use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct FlowSelector;

// after all entities have spawned, check which selectors are needed, emit those events.
pub fn outflow_product_energy_existence_dispatch(
        mut commands: Commands,
        query: Query<Entity, With<SOI>>,
        children_query: Query<&Children>
) {
        
        let soi = query.single();
        info!("# of children_children {:?}", children_query.iter().len());
        
        for child in children_query.iter_descendants(soi) {
                info!("{:?} is a descendant of {:?}", child, soi);
        }
}

/*  DSA Syntax & Method Existence Validation
        Given a System(S) at Level(T),
        Does it have #N of With<Components(C)>?
*/

/* Interactions (e.g., Flows)
ExternalInterfactions
        From: (Src/Snk)
        To:   (System)
InternalInteractions
        From: (System)
        To:   (System)
*/