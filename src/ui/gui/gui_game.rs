use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{
	components::resources::resources, 
	make_text_variants,
    make_bkg_variants,
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

    make_bkg_variants(
        "",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/panel_bkg.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				// each field of a rect defines the size of a frame on each side of the source image
				source: Rect { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0},
				// destination maps source rect frame to the ui space values so if you don't want to "rescale" the frame just make it the same as source.
				// if you double the values here then all frame sides would have doubled size in ui space.
				destination: Rect { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
    );

	make_bkg_variants(
        "data",
        ThemedImageMaterial::Image(ImageBoxImage {
            id: "ui/panel_data.png".to_owned(),
            scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				// each field of a rect defines the size of a frame on each side of the source image
				source: Rect { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0},
				// destination maps source rect frame to the ui space values so if you don't want to "rescale" the frame just make it the same as source.
				// if you double the values here then all frame sides would have doubled size in ui space.
				destination: Rect { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0},
				..Default::default()
			  }),
            ..Default::default()
        }),
        &mut theme.content_backgrounds,
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