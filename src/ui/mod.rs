use oxygengine::user_interface::raui::core::prelude::*;
use oxygengine::user_interface::raui::material::prelude::*;
use std::collections::HashMap;

pub mod gui;
pub mod components;

pub fn setup(app: &mut Application) {
    app.register_props::<components::menu_btn::MenuBtnProps>("MenuBtnProps");
    
    app.register_props::<components::splash::SplashState>("SplashState");
    app.register_props::<components::splash::SplashTextProps>("SplashTextProps");
    app.register_component("gui_splash", gui::gui_splash::gui_splash);

    app.register_props::<components::menu::MenuTextProps>("MenuTextProps");
    app.register_component("gui_menu", gui::gui_menu::gui_menu);
}

pub fn new_theme() -> ThemeProps {
    let mut theme = make_default_theme(
        color_from_rgba(255,255,255, 1.),
        color_from_rgba(255, 255, 255, 1.),
        color_from_rgba(255, 255, 255, 1.),
        color_from_rgba(255, 255, 255, 1.),
    );
    make_text_variants(
        "",
        ThemedTextMaterial {
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 18.,
            },
            alignment: TextBoxAlignment::Center,
            ..Default::default()
        },
        &mut theme.text_variants,
    );
    theme
}


fn make_text_variants(
    base_id: &str,
    base_material: ThemedTextMaterial,
    text_variants: &mut HashMap<String, ThemedTextMaterial>,
) {
    text_variants.insert(base_id.to_owned(), base_material);
}
