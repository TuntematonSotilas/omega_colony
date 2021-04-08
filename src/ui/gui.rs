use oxygengine::user_interface::raui::core::prelude::*;

use crate::ui::{
    new_theme,
    components::centered_text::splash,
};

widget_component! {
    pub gui(key, named_slots) {
        widget! {
            (#{key} content_box | {new_theme()} [
                (#{"splash"} splash)
            ])
        }
    }
}
