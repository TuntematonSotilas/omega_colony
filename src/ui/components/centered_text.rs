use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CenteredTextProps {
    #[serde(default)]
    pub label: String,
}
implement_props_data!(CenteredTextProps);


widget_component! {
    pub centered_text(key, props) {
        let spl_props = props.read_cloned_or_default::<CenteredTextProps>();
        let label_props = TextPaperProps {
            text: spl_props.label.to_owned(),
            variant: "".to_owned(),
           	use_main_color: true,
            alignment_override: Some(TextBoxAlignment::Left),
            ..Default::default()
        };

        widget! {
            (#{key} horizontal_box: {props.clone()} [
                (#{"label"} text_paper: {label_props})
            ])
        }
    }
}

widget_component! {
    pub splash(key) {
        widget! {
            (#{key} centered_text: {CenteredTextProps { label: "Splash".to_owned() }})
        }
    }
}