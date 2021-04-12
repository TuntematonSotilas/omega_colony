use oxygengine::user_interface::raui::core::prelude::*;

use crate::ui::{
    new_theme,
    components::intro::intro,
};

widget_component! {
    pub gui_intro(key, named_slots) {
        widget! {
            (#{key} content_box | {new_theme()} [
                (#{"intro"} intro)
            ])
        }
    }
}