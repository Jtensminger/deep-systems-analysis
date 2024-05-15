use crate::components::{InitialPosition, SubstanceType};
use crate::constants::FLOW_LENGTH;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CreateButton {
    pub ty: CreateButtonType,
    pub connection_source: Entity,
    pub system: Entity,
    pub substance_type: Option<SubstanceType>,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum CreateButtonType {
    ImportInterface,
    ExportInterface,
    Inflow,
    Outflow,
    Source,
    Sink,
    InterfaceSubsystem { is_child_of_interface: bool },
    FlowTerminalStart,
    FlowTerminalEnd,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasFlowInterfaceButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasFlowOtherEndButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasInterfaceSubsystemButton {
    pub button_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct FlowCurve {
    pub start: Vec2,
    pub end: Vec2,
    pub start_direction: Vec2,
    pub end_direction: Vec2,
}

impl FlowCurve {
    pub fn head_rotation(&self) -> Quat {
        Quat::from_rotation_z(self.end_direction.to_angle())
    }

    pub fn inflow(
        zoom: f32,
        initial_position: InitialPosition,
        direction: Vec2,
        scale: f32,
    ) -> Self {
        Self {
            start: (*initial_position + direction * FLOW_LENGTH * scale) * zoom,
            start_direction: -direction,
            end: *initial_position * zoom,
            end_direction: direction,
        }
    }

    pub fn outflow(
        zoom: f32,
        initial_position: InitialPosition,
        direction: Vec2,
        scale: f32,
    ) -> Self {
        Self {
            start: *initial_position * zoom,
            start_direction: direction,
            end: (*initial_position + direction * FLOW_LENGTH * scale) * zoom,
            end_direction: -direction,
        }
    }

    #[inline]
    pub fn compute_tangent_length(&self) -> f32 {
        Self::compute_tangent_length_from_points(self.start, self.end)
    }

    #[inline]
    pub fn compute_tangent_length_from_points(start: Vec2, end: Vec2) -> f32 {
        (end - start).length() * 0.3333
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlowTerminalSelecting {
    Start,
    End,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SelectedHighlightHelperAdded {
    pub helper_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct Pin {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct Pinnable {
    pub has_pins: bool,
}
