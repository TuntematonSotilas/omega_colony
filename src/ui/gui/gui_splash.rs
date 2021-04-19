use oxygengine::user_interface::raui::core::prelude::*;

use crate::ui::components::splash::splash;

widget_component! {
    pub gui_splash(key, named_slots) {
        widget! {
            (#{key} content_box [
                (#{"splash"} splash)
            ])
        }
    }
}
