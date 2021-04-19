use oxygengine::user_interface::raui::core::prelude::*;

use crate::ui::{
    menu_theme,
    components::menu::menu,
};

widget_component! {
    pub gui_menu(key, named_slots) {
        widget! {
            (#{key} content_box | {menu_theme()} [
                (#{"menu"} menu)
            ])
        }
    }
}