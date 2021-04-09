use oxygengine::user_interface::raui::core::prelude::*;
use oxygengine::user_interface::raui::material::prelude::*;
use std::collections::HashMap;

pub mod gui;
pub mod components;

pub fn setup(app: &mut Application) {
    app.register_props::<components::centered_text::CenteredTextProps>("CenteredTextProps");
    app.register_component("gui", gui::gui);
}

pub fn new_theme() -> ThemeProps {
    let mut theme = make_default_theme(
        color_from_rgba(255,220,78, 1.0),
        color_from_rgba(255, 255, 255, 1.0),
        color_from_rgba(255, 255, 255, 1.0),
        color_from_rgba(255, 255, 255, 1.0),
    );
    make_text_variants(
        "",
        ThemedTextMaterial {
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 16.0,
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
