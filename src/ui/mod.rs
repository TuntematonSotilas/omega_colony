pub mod gui;
pub mod screens;

use oxygengine::user_interface::raui::core::prelude::*;
use oxygengine::user_interface::raui::material::prelude::*;

pub fn setup(app: &mut Application) {
    app.register_component("gui", gui::gui);
    app.register_component("splash", screens::splash::splash);
}

pub fn new_theme() -> ThemeProps {
    let theme = new_all_white_theme();
    theme
}
