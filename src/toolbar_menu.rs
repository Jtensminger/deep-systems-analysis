use bevy::prelude::*;

use bevy_egui::{EguiContexts, EguiPlugin};
// use bevy::ui::*;

pub struct ToolbarMenuPlugin;

impl Plugin for ToolbarMenuPlugin {
        fn build(
                &self,
                app: &mut App
        ) {
                app.add_plugins(EguiPlugin);
                app.add_systems(Update, setup_toolbar_menu);
        }
}

fn setup_toolbar_menu(
        mut contexts: EguiContexts               // EguiContexts is a Bevy Resource that holds the EguiContext
) {
        let ctx = contexts.ctx_mut();           // get a mutable reference to the EguiContext
        let valid_types = vec!["System", "Flow", "Interface", "Source", "Sink"];
        

        egui::Window::new("Toolbar Menu")
                .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::new(0., -200.0))
                .title_bar(false)              
                .show(ctx, |ui| {          
                        for t in valid_types {
                                ui.button(t);                               
                        }
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