use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{components::menu::menu, make_text_variants, make_button_variants, make_bkg_variants};

fn theme_menu() -> ThemeProps {
    let mut theme = make_default_theme(
        color_from_rgba(255,255,255, 1.),
        color_from_rgba(255, 255, 255, 1.),
        color_from_rgba(0, 153, 255, 1.),
        color_from_rgba(255, 255, 255, 1.),
    );

    make_text_variants(
        "",
        ThemedTextMaterial {
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 18.,
            },
            alignment: TextBoxAlignment::Left,
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
				scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
					source: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					destination: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					..Default::default()
				  }),
                ..Default::default()
            }),
             selected: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
				scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
					source: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					destination: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					..Default::default()
				  }),
                tint: {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.9 }
                },
                ..Default::default()
            }),
            trigger: ThemedImageMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
				scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
					source: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					destination: Rect { left: 3.0, right: 3.0, top: 3.0, bottom: 3.0},
					..Default::default()
				  }),
                tint: {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.7 }
                },
                ..Default::default()
            }),
            ..Default::default()
        },
        &mut theme.button_backgrounds,
    );

	make_bkg_variants(
        "",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/stars.png".to_owned(),
            scaling: ImageBoxImageScaling::Stretch,
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

    theme
}

pub fn gui_menu(context: WidgetContext) -> WidgetNode {
    widget! {
        (#{context.key} content_box | {theme_menu()} [
            (#{"menu"} menu)
        ])
    }
}