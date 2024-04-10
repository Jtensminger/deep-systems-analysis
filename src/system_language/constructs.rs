use bevy::prelude::*;
use std::fmt;


#[derive(Component, Default)]
pub struct Construct {
        pub construct_properties: ConstructProperties,
        pub z: usize
}
impl Construct {
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
        pub construct_set_type: ConstructSetType
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
                }
        }
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

