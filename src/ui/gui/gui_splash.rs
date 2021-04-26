use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use crate::ui::{components::splash::splash, make_bkg_variants};

pub fn theme_splash() -> ThemeProps {
	let mut theme = make_default_theme(
		color_from_rgba(255,255,255, 1.),
		color_from_rgba(255, 255, 255, 1.),
		color_from_rgba(0, 153, 255, 1.),
		color_from_rgba(255, 255, 255, 1.),
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


widget_component! {
    pub gui_splash(key, named_slots) {
        widget! {
            (#{key} content_box| {theme_splash()} [
                (#{"splash"} splash)
            ])
        }
    }
}
