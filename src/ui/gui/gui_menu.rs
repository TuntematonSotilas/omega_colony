use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{components::menu::menu, make_text_variants, make_button_variants};

pub fn theme_menu() -> ThemeProps {
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

    make_text_variants(
        "btn",
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

    make_button_variants(
        "",
        ThemedButtonMaterial {
            default: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
                ..Default::default()
            }),
             selected: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
                tint: {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.9 }
                },
                ..Default::default()
            }),
            trigger: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
                tint: {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 }
                },
                ..Default::default()
            }),
            ..Default::default()
        },
        &mut theme.button_backgrounds,
    );
    theme
}

widget_component! {
    pub gui_menu(key, named_slots) {
        widget! {
            (#{key} content_box | {theme_menu()} [
                (#{"menu"} menu)
            ])
        }
    }
}