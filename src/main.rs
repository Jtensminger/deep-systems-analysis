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
        prelude::*,
        math::bounding::*,
        sprite::MaterialMesh2dBundle,
        window::PrimaryWindow
        //input::mouse::MouseButton,
};

// DSA dependencies
use system_language::{
        SystemLanguagePlugin,
        CoordinatePosition
};
use toolbar_menu::ToolbarMenuPlugin;
use camera::CameraPlugin;

// Dev dependencies
use std::env;

fn main() {
        //env::set_var("RUST_BACKTRACE", "1");
        App::new()                                          // Creates a new App with some default structure to enable core engine features
                // set global state (e.g., Resources)
                .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6))) // default background color

                // Engine & Development Plugins
                .add_plugins(DefaultPlugins)                // adds default bevy engine features (aka plugins)

                // App Plugins
                .add_plugins(ToolbarMenuPlugin)            
                .add_plugins(CameraPlugin)
                .add_plugins(SystemLanguagePlugin)

                // App Systems
                //.add_systems(Startup, (spawn_system_of_interest, draw_system_of_interest).chain())                                         // Add a Circle to the Canvas
                //.add_systems(Update, draw_gizmos_2d)                                                      
                
                //.add_systems(Update, grab_mouse)                                                      // Toggles Cursor to Grab or Grabbing

                .run();                                     // Starts the application by calling the app’s runner function. Finalizes the App configuration.
}

/*
fn draw_gizmos_2d(
        mut gizmos: Gizmos
) {
        let position: Vec2 = CoordinatePosition::center().to_vec2();
        let color = Color::WHITE;
        let angle = 90.;

        let shape = Circle { 
                radius: 100.
        };

        let bounding = BoundingCircle {
                center: position,
                circle: shape,
        };
        
        let spatial_bundle = SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..Default::default()
        };

        // draw a circle
        gizmos.primitive_2d(shape, position, angle, color); 
        
        // draw a line
        let segment = bevy::prelude::Segment2d::new(Direction2d::from_xy(1., 0.3).unwrap(), 90.);
        gizmos.primitive_2d(segment, Vec2::new(0., 0.), 0., Color::WHITE);
}
*/







// https://bevy-cheatbook.github.io/input/mouse.html
/* 
fn grab_mouse(
        mut windows: Query<&mut Window>,
        mouse: Res<ButtonInput<MouseButton>>,
) {
        let mut window = windows.single_mut(); // queriering to retreive the mutuatable window entity

        /* change mouse icon to grab or grabbing*/
        /* event order goes Input::pressed -> Input::just_pressed -> Input::just_released */
        if mouse.pressed(MouseButton::Left) {
                window.cursor.icon = CursorIcon::Grabbing;
                info!("left mouse currently pressed");
        }
        
        if mouse.just_pressed(MouseButton::Left) { // Input::just_pressed will return true for one frame after a press event
                info!("left mouse just pressed");
        }
        
        if mouse.just_released(MouseButton::Left) { // Input::just_released will return true for one frame after a release event
                window.cursor.icon = CursorIcon::Grab;
                info!("left mouse just released");
        }
}
*/
