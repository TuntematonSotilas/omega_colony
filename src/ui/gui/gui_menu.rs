use oxygengine::user_interface::raui::core::prelude::*;

use crate::ui::{
    new_theme,
    components::menu::menu,
};

widget_component! {
    pub gui_menu(key, named_slots) {
        widget! {
            (#{key} content_box | {new_theme()} [
                (#{"menu"} menu)
            ])
        }
    }
}