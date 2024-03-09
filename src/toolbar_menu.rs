use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
// helper crate to use e-gui to tweak UI as we build it. Can be removed at the end.
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct ToolbarMenuPlugin;

impl Plugin for ToolbarMenuPlugin {
        fn build(
                &self,
                app: &mut App
        ) {
                app
                        .add_plugins(EguiPlugin)                    // Adds all Egui resources and render graph nodes.
                        .add_plugins(WorldInspectorPlugin::new())   // adds inspector plugin to tweak UI as we build it. Can be removed at the end.

                        .add_systems(Update, setup_toolbar_menu);
        }
}

fn setup_toolbar_menu(
        mut contexts: EguiContexts               // EguiContexts is a Bevy Resource that holds the EguiContext
) {
        let valid_menu_options = vec!["System", "Flow", "Interface", "Source", "Sink", "TBD"];
        
        egui::Window::new("Toolbar Menu")
                .title_bar(false)
                .movable(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
                .frame(
                        egui::Frame::default()
                                .fill(egui::Color32::WHITE)
                                .stroke(egui::Stroke::new(1., egui::Color32::from_rgba_premultiplied(0, 0, 0, 100)))
                                .rounding(10.)
                                .shadow(egui::epaint::Shadow::small_light())
                                .inner_margin(10.)
                                .outer_margin(15.)
                )
                .show(contexts.ctx_mut(), |ui| {
                        ui.allocate_ui_with_layout(egui::vec2(600.0, 50.0), egui::Layout::left_to_right(egui::Align::Center), |option_container| {
                                for menu_option in valid_menu_options {
                                        let menu_option_text = egui::widget_text::WidgetText::RichText(
                                                egui::RichText::new(menu_option.to_string())
                                                        .color(egui::Color32::BLACK)                      
                                        );
                                        let button = egui::Button::new(menu_option_text)
                                                .fill(egui::Color32::LIGHT_GRAY)
                                                .stroke(egui::Stroke::new(1., egui::Color32::BLACK))
                                                .rounding(10.);
                                        option_container.add_sized([50., 50.], button);            
                                }
                        });
                });
        
}

/* BevyUI Implementation 
fn setup_toolbar_menu(
        mut commands: Commands
) {
        // create a new UI Node Bundle to hold our ToolbarMenu Buttons
        commands.spawn(NodeBundle {
                background_color: BackgroundColor(Color::rgba(0.6, 0.6, 1.0, 1.0)), // light blue
                border_color: BorderColor(Color::rgba(0.8, 0.8, 0.8, 1.0)),         // light grey
                focus_policy: FocusPolicy::Pass,                                    // pass focus policy so items in the toolbar can be clicked
                visibility: Visibility::Visible,                                    // make the toolbar visible 
                style: Style { 
                        // Positioning the toolbar container
                        display:       Display::Flex,          // using Flexbox to layout the toolbar
                        position_type: PositionType::Absolute, // using Absolute positioning to layout the toolbar
                        width:         Val::Vh(20.0),          // using viewport width
                        //min_width:     Val::Px(200.0),  
                        max_width:     Val::Vh(10.0),         
                        min_height:    Val::Vh(7.0),           // using viewport height
                        max_height:    Val::Vh(10.0),
                        bottom:        Val::Px(50.0),         
                        left:          Val::Px(50.0),
                        // Positioning the toolbar items
                        // [TODO]
                        ..Default::default()
                },   
                ..Default::default()
                });
}
*/