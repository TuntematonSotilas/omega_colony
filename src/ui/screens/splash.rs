use oxygengine::user_interface::raui::core::prelude::*;

widget_component! {
    pub splash(key) {
        widget!{{{
            TextBoxNode {
                text: "splash".to_owned(),
                ..Default::default()
            }
        }}}
    }
}