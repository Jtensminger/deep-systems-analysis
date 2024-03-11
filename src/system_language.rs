use std::fmt;
use bevy::{
        asset::load_internal_asset, prelude::*,
        render::render_resource::{AsBindGroup, ShaderRef},
        sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
        window::PrimaryWindow
};

pub const CUSTOM_MATERIAL_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(3253086872234592509);

pub struct SystemLanguagePlugin;

impl Plugin for SystemLanguagePlugin {
        fn build(&self,app: &mut App) {
                load_internal_asset!(
                        app,
                        CUSTOM_MATERIAL_SHADER_HANDLE,
                        "shaders/custom_material.wgsl",
                        Shader::from_wgsl
                );            
                app.add_plugins(Material2dPlugin::<CustomMaterial>::default())
                        .init_resource::<SelectedConstruct>()
                        .init_resource::<ConstructRenderData>()
                        .add_systems(Startup, setup_construct)
                        .add_systems(PostUpdate, on_spawn_construct);

                app.world.resource_mut::<Assets<CustomMaterial>>().insert(
                        Handle::<CustomMaterial>::default(),
                        CustomMaterial {
                                color: Color::WHITE,
                                ..Default::default()
                        },
                );
                    
        }
}

#[derive(Resource)]
pub struct ConstructRenderData {
        /* 
                Construct rendering data is stored in a resource.
                This is basically the place for a global lookup of what your card needs to look like based on what type of construct
        */
        system_of_interest_mesh: Mesh2dHandle,
        system_of_interest_material: Handle<CustomMaterial>,
        // component_mesh: Handle<Mesh>,
        // external_construct_mesh: Handle<Mesh>,
        // flow_mesh: Handle<Mesh>,
        // interface_mesh: Handle<Mesh>,
        // component_base_material: Handle<StandardMaterial>,
}
impl ConstructRenderData {
        pub fn placeholder_mesh() -> Mesh {
                Circle::new(300.)
                        .mesh()
                        .resolution(128)
                        .build()
        }
        pub fn mesh_by_type(&self, construct_set_type: ConstructSetType) -> Mesh2dHandle {
                match construct_set_type {
                        _ | ConstructSetType::SystemOfInterest => self.system_of_interest_mesh.clone(),
                        // ConstructSetType::Component => self.component_mesh.clone(),
                        // ConstructSetType::Boundary  => self.component_mesh.clone(),
                        // ConstructSetType::Interface => self.component_mesh.clone(),
                        // ConstructSetType::Flow      => self.component_mesh.clone(),
                        // ConstructSetType::Source | ConstructSetType::Sink => self.component_mesh.clone(),
                }
        }
        pub fn material_by_type(&self, construct_set_type: ConstructSetType) -> Handle<CustomMaterial> {
                match construct_set_type {
                        _ | ConstructSetType::SystemOfInterest => self.system_of_interest_material.clone(),
                        // ConstructSetType::Component => self.component_base_material.clone(),
                        // ConstructSetType::Boundary  => self.component_base_material.clone(),
                        // ConstructSetType::Interface => self.component_base_material.clone(),
                        // ConstructSetType::Flow      => self.component_base_material.clone(),
                        // ConstructSetType::Source | ConstructSetType::Sink => self.component_base_material.clone(),
                }
        }
}
impl FromWorld for ConstructRenderData {
        /* 
                FromWorld enables ConstructRenderData to be constructed using data from the supplied World. 
                It's helpful for complex initialization or context-aware defaults
        */
        fn from_world(world: &mut World) -> Self {
                let world = world.cell();
                let mut meshes = world.resource_mut::<Assets<Mesh>>();
                let mut materials = world.resource_mut::<Assets<CustomMaterial>>();
                Self {
                        system_of_interest_mesh: Mesh2dHandle(meshes.add(ConstructRenderData::placeholder_mesh())),
                        system_of_interest_material: materials.add(CustomMaterial {
                                color: Color::WHITE,
                                ..Default::default()
                        }),
                }
    }
}



#[derive(Component, Default)]
pub struct Construct {
        pub construct_properties: ConstructProperties,
        pub z: usize,
}
impl Construct {
        // pub const ASPECT_RATIO: f32 = 50.0 / 60.0;
        // pub const ART_WIDTH: f32 = 167.0;
        // pub const ART_HEIGHT: f32 = 166.0;
        // pub const ART_ASPECT: f32 = Self::ART_WIDTH / Self::ART_HEIGHT;
        // pub const SPAWN_OFFSET: f32 = 1.0;
    
        pub fn construct_set_type(&self) -> ConstructSetType {
                self.construct_properties.construct_set_type
        }
}
impl From<ConstructSetType> for Construct {
        /* allows us to build a Construct using just the ConstructSetType  */
        fn from(construct_set_type: ConstructSetType) -> Self {
                Self {
                        construct_properties: construct_set_type.into(),
                        ..default()
                }
        }
}


pub struct ConstructProperties {
        /* TBD but I think all Construct structural aspects go here. Knowledgebase goes elsewhere shared across all types go here.*/
        pub construct_set_type: ConstructSetType,
        pub construct_set_complexity: ConstructSetComplexity,        
}
impl Default for ConstructProperties {
        fn default() -> Self {
                ConstructSetType::default().into()
        }
}
impl From<ConstructSetType> for ConstructProperties {
        fn from(construct_set_type: ConstructSetType) -> Self {
                Self {
                        construct_set_type,
                        construct_set_complexity: ConstructSetComplexity::Atomic,
                }
        }
}


#[derive(Default, Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum ConstructSetComplexity {
        /* used to determine composability, among other things */
        #[default]
        Atomic,
        Complex,
        Multiset /* bounded to hold many instances of that same type of component */
}


#[derive(Default, Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum ConstructSetType {
        /* All Foundational System Language Types  */
        SystemOfInterest,
        #[default]
        Component,
        Boundary, /* <-- Unclear is this will stay here. */
        Interface,
        Source,
        Sink,
        Flow,
}
impl fmt::Display for ConstructSetType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self)
        }
}


#[derive(Bundle)]
pub struct ConstructBundle {
        pub construct: Construct,
        pub material_mesh_2d_bundle: MaterialMesh2dBundle<CustomMaterial>,
        /*  Rapier Physics Fields */
        // pub collider: Collider,
        // pub sensor: Sensor,
        // pub rigid_body: RigidBody,
        // pub active_events: ActiveEvents,
        // pub active_collision_types: ActiveCollisionTypes,
}
impl Default for ConstructBundle {
        fn default() -> Self {
                Self {
                        construct: Default::default(),
                        material_mesh_2d_bundle: MaterialMesh2dBundle::default(),
                        /*  Rapier Physics Fields */
                        //collider: Collider::cuboid(Card::ASPECT_RATIO / 2.0, 1.0 / 2.0, 0.2),
                        //sensor: Sensor,
                        //active_events: ActiveEvents::COLLISION_EVENTS,
                        //active_collision_types: ActiveCollisionTypes::all(),
                        //rigid_body: RigidBody::Fixed,
                }
        }
}

#[derive(Resource, Default, PartialEq, Eq, Copy, Clone)]
pub enum SelectedConstruct {
        /* The state of whether the user currently has selected the Construct*/
        Some(Entity),
        #[default]
        None,
}
impl SelectedConstruct {
        fn is_selected(self, entity: Entity) -> bool {
                match self {
                        SelectedConstruct::Some(e) => e == entity,
                        SelectedConstruct::None => false,
                }
        }
}

/* Not Needed Yet
#[derive(Default)]
pub enum HoverPoint {
        Some(Vec3),
        #[default]
        None,
}
*/

pub struct CoordinatePosition {
        pub x: f32,
        pub y: f32,
        pub z: f32
}
impl CoordinatePosition {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
                Self { x, y, z }
        }

        pub fn center() -> Self {
                Self { x: 0., y: 0., z: 0. }
        }

        pub fn x(mut self, x: f32) -> Self {
                self.x = x;
                self
        }

        pub fn y(mut self, y: f32) -> Self {
                self.y = y;
                self
        }

        pub fn z(mut self, z: f32) -> Self {
                self.z = z;
                self
        }

        pub fn to_vec3(&self) -> Vec3 {
                Vec3::new(self.x, self.y, self.z)
        }

        pub fn to_vec2(&self) -> Vec2 {
                Vec2::new(self.x, self.y)
        }
}

fn setup_construct(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>, /* idk if i need to use this to modify/update the SOI asset or not. */
        render_data: ResMut<ConstructRenderData>,
        window_query: Query<&Window, With<PrimaryWindow>>
) {
        // get the old SOI asset and replace it with the new one based on screen size
        if let Some(mesh) = meshes.get_mut(render_data.system_of_interest_mesh.0.id()) {
                let primary_window = window_query.single();
                let (mut available_logical_width, mut available_logical_height) = (primary_window.resolution.width(), primary_window.resolution.height());        
                let padding_percentage = 0.2;
                available_logical_height -= available_logical_height * padding_percentage;
                available_logical_width  -= available_logical_width  * padding_percentage;
                let max_radius = f32::min(available_logical_height, available_logical_width) / 2.;
                *mesh = Circle::new(max_radius)
                        .mesh()
                        .resolution(256)
                        .build();
        }
        let construct_bundle = ConstructBundle {
                construct: Construct::from(ConstructSetType::SystemOfInterest),
                material_mesh_2d_bundle: MaterialMesh2dBundle {
                        material: render_data.material_by_type(ConstructSetType::SystemOfInterest).clone(),
                        mesh: render_data.system_of_interest_mesh.clone(),
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                },
        };

        commands.spawn(construct_bundle);
}

/* responsible for spawning the constructs along with all their children */
fn on_spawn_construct(
        mut commands: Commands,                                     //  Command represents some mutation we want to perform on our world. Bevy takes these commands and will apply them to our world.
        render_data: Res<ConstructRenderData>,            // materials and meshes
        constructs: Query<(Entity, &Construct), Added<Construct>>,  // A filter on a component that only retains results added after the system last ran.
) {
        // for (entity, construct) in &constructs {                        // A list of entities and their components that have been added since the last system run.
        //         commands.entity(entity)                                 /* A list of commands that will be run, but haven't yet ran, to modify an entity. */
        //                 .with_children(|parent| {              
        //                         /*
        //                                 .with_children() will create new entities, which are called child entities.
        //                                 It'll update the newly created child entity's Parent component to hold the parent entity.
        //                                 It'll add the child entity to the parent_entity's Children component, which is a SmallVec<Entity>
        //                         */
        //                         /* visuals */
        //                         // parent.spawn(PbrBundle {
        //                         //         material: render_data.material_by_type(construct.construct_set_type()).clone(),
        //                         //         mesh: render_data.mesh_by_type(construct.construct_set_type()).clone(),
        //                         //         transform: Transform::from_xyz(0., 0., 0.),
        //                         //         ..default()
        //                         // });
        //                         /* spatial */
        //                         // parent.spawn(SpatialBundle::default());
        //                                 // .with_children(|parent| {
        //                                 //         for i in 0..max {
        //                                 //                 parent.spawn_bundle(PbrBundle {
        //                                 //                         material: card_data.heart_material.clone(),
        //                                 //                         mesh: card_data.heart_mesh.clone(),
        //                                 //                         transform: Transform::from_xyz(
        //                                 //                                 i as f32 * offset - width / 2.0,
        //                                 //                                 0.37,
        //                                 //                                 0.01,
        //                                 //                         ),
        //                                 //                         ..default()
        //                                 //                 });
        //                                 //         }
        //                                 // });
        //                 });
        // }
}

// This struct defines the data that will be passed to your shader
#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct CustomMaterial {
        // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
        // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
        #[uniform(0)]
        color: Color,
        // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
        // add the sampler attribute with a different binding index.
        #[texture(1)]
        #[sampler(2)]
        texture: Option<Handle<Image>>,
}
impl Default for CustomMaterial {
        fn default() -> Self {
                CustomMaterial {
                        color: Color::WHITE,
                        texture: None,
                }
        }
}
    
/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
        fn fragment_shader() -> ShaderRef {
                CUSTOM_MATERIAL_SHADER_HANDLE.into()
        }
}




/* 
Mesh -> textured -> rendered
Problem:
        How to position systems in side other system? (Treat it like a cirlce?)
        How to position an interface on a system? (Treat it like a Ring?)?
        How do we change the interior color of a system?
        How do we change the perceived border/stroke color of a system?
Proposed solution: 
        Your Position component could have a "parent/children" logic,
        where any Entity with a Position may have a parent and their position is relative to their parent.
        Instead of having several meshs on the same entity, you can make more than one entity, each with its own mesh and link them together.
        You can even make the children entities listen to their parent events (or whatever system you have for communication between entities) and react accordingly
        Regions we care about:
        - The boundary area of the system where interfaces are placed.
        - Inside the system but not in the boundary ring where everything else can be placed.
        Child entities could be:
        - Construct Interior Circle
        - Construct Boundary Ring
Border/Color Issues:
        Option 1: 2x Meshes, 2x different Color Materials
        Option 2: 1x Mesh, Custom Shader to draw stroke.
In all scenarios, we need to ability to change the interior color and shape outline color, so a fragment shader is needed to generate outlines.
*/