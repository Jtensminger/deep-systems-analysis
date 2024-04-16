use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

mod levels;
use levels::*;

mod flows;
use flows::*;

mod properties;
use properties::*;

mod selector;
use selector::*;

mod system;
use system::*;

const SOI_RADIUS: f32 = 300.0;
const SOI_FILL_COLOR: Color = Color::ORANGE_RED;
const SOI_STROKE_COLOR: Color = Color::BLACK;
const SOI_STROKE_SIZE: f32 = 5.0;

pub struct SystemLanguagePlugin;
impl Plugin for SystemLanguagePlugin {
        fn build(&self,app: &mut App) {
                app
                        .add_plugins(ShapePlugin)

                        .add_event::<construct_spawned::SOI>()
                        .add_event::<construct_spawned::Boundary>()
                        .add_event::<construct_spawned::Environment>()

                        .add_systems(Startup, spawn_soi)
                        .add_systems(Update, (
                                handle_soi_spawn,
                                outflow_product_energy_existence_dispatch,
                                draw_soi
                        ).chain());
        }
}
