use oxygengine::user_interface::raui::{
	core::prelude::*,
	material::prelude::*,
};
use std::collections::HashMap;

pub mod gui;
pub mod components;

pub fn setup(app: &mut Application) {
    app.register_props::<components::splash::SplashState>("SplashState");
    app.register_props::<components::splash::SplashTextProps>("SplashTextProps");
    app.register_component("gui_splash", gui::gui_splash::gui_splash);

    app.register_props::<components::menu_btn::MenuBtnProps>("MenuBtnProps");
    app.register_props::<components::menu::MenuState>("MenuState");
    app.register_props::<components::menu::MenuTextProps>("MenuTextProps");
    app.register_component("gui_menu", gui::gui_menu::gui_menu);

	app.register_component("gui_game", gui::gui_game::gui_game);
}

fn make_text_variants(
    base_id: &str,
    base_material: ThemedTextMaterial,
    text_variants: &mut HashMap<String, ThemedTextMaterial>,
) {
    text_variants.insert(base_id.to_owned(), base_material);
}

fn make_button_variants(
    base_id: &str,
    base_material: ThemedButtonMaterial,
    btn_variants: &mut HashMap<String, ThemedButtonMaterial>,
) {
    btn_variants.insert(base_id.to_owned(), base_material);
}

fn make_bkg_variants(
    base_id: &str,
    base_material: ThemedImageMaterial,
    bkg_variants: &mut HashMap<String, ThemedImageMaterial>,
) {
    bkg_variants.insert(base_id.to_owned(), base_material);
}