use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{
	components::resources::resources, 
	make_text_variants
};


pub fn theme_game() -> ThemeProps {
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
                size: 12.,
            },
            alignment: TextBoxAlignment::Center,
            ..Default::default()
        },
        &mut theme.text_variants,
    );
    theme
}

widget_component! {
    pub gui_game(key, named_slots) {
        widget! {
            (#{key} content_box | {theme_game()} [
                (#{"resources"} resources)
            ])
        }
    }
}