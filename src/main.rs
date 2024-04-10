/*  App
        [app world] is the main ECS World of the App. Stores and exposes operations on entities, components, resources, and their associated metadata.
        [app runner] function is primarily responsible for managing the application’s event loop and advancing the Schedule.
        [app main_schedule_label] is the schedule that systems are added to by default & that runs the main loop of schedule execution.
*/

/* [app world]
        Each Entity (aka uuid) has a set of components (aka data).
        Each component can have up to one instance of each component type.
        Entity components can be created, updated, removed, and queried using a given World.
        To mutate different parts of the world simultaneously, use World::resource_scope or SystemState.
        Worlds can also store Resources, which are unique instances of a given type that don’t belong to a specific Entity.
*/

/* [bevy_ecs]
        [Example] https://github.com/bevyengine/bevy/blob/latest/examples/ecs/ecs_guide.rs#L73
        [Components] are normal Rust structs. They are data stored in a World and specific instances of Components correlate to Entities.
        [World] Entities, Components, and Resources are stored in a World. Worlds, much like Rust std collections like HashSet and Vec, expose operations to insert, read, write, and remove the data they store.
        [Entities] Entities are unique identifiers that correlate to zero or more Components.
*/

/* [bevy_ecs States] https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.States.html
        Types that can define world-wide states in a finite-state machine
*/

/* [bevy_ecs Resources]
        Represent globally unique pieces of data stored in the ecs world
        Resources consist of static data, but not constant data.
        Types must be unique, and there can only be at most one instance of a given type. If multiple instances are needed, consider using entities and components instead. 
*/

/* Canvas
        DragCanvas: TouchPad::TwoFingerDrag or Mouse::RightClick
        BackgroundColor: Off-white
*/

/* ToolBarUI
        SelectToolBarItem: OnHover --> Highlight Item in Toolbar
        SelectToolBarItem: OnClick --> CursorMode changes to specific CursorMode::EntityType
        CursorMode::EntityType: OnClick --> Drops EntityType on Canvas
*/
mod system_language;
mod toolbar_menu;
mod camera;

// Bevy dependencies
use bevy::{
        math::bounding::*,
        prelude::*,
        sprite::{Material2dPlugin, MaterialMesh2dBundle},
        window::PrimaryWindow
};

// DSA dependencies
use system_language::SystemLanguagePlugin;
use toolbar_menu::ToolbarMenuPlugin;
use camera::CameraPlugin;

// Dev dependencies
use std::env;

fn main() {
        //env::set_var("RUST_BACKTRACE", "1");
        App::new()  // Creates a new App with some default structure to enable core engine features
                
                // set global state (e.g., Resources)
                .insert_resource(ClearColor(Color::ANTIQUE_WHITE)) // default background color
                .insert_resource(Msaa::Sample4)

                // Engine & Development Plugins
                .add_plugins(DefaultPlugins) // adds default bevy engine features (aka plugins)

                // App Plugins
                .add_plugins(ToolbarMenuPlugin)            
                .add_plugins(CameraPlugin)
                .add_plugins(SystemLanguagePlugin)

                .run();
}